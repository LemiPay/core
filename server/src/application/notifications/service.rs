use std::sync::Arc;

use crate::application::common::repo_error::RepoError;
use crate::application::group::traits::repository::GroupRepository;
use crate::application::notifications::repository::NotificationRepository;
use crate::application::users::traits::repository::UserRepository;
use crate::domain::group::GroupId;
use crate::domain::notification::types::{NotificationChannelId, NotificationEventId};
use crate::domain::user::Email as DomainEmail;
use crate::domain::user::UserId;
use crate::infrastructure::email::email_sender::EmailService;
use crate::infrastructure::notification::persistent_service::{
    PersistentNotificationError, PersistentNotificationService,
};

/// Central notification dispatcher (the "observer").
/// Dispatches to email and/or web channels based on user and group preferences.
#[derive(Clone)]
pub struct NotificationService {
    pub notification_repo: Arc<dyn NotificationRepository>,
    pub email_service: Arc<dyn EmailService>,
    pub persistent_notification_service: Arc<dyn PersistentNotificationService>,
    pub group_repo: Arc<dyn GroupRepository>,
    pub user_repo: Arc<dyn UserRepository>,
}

impl NotificationService {
    /// Notify about a group-scoped event (the main entry point for most business events).
    /// The service expands to active group members, checks their prefs (group then user level)
    /// per channel, and dispatches accordingly.
    pub async fn notify_group_event(&self, event_name: &str, group_id: GroupId) {
        if let Err(e) = self.notify_group_event_inner(event_name, group_id).await {
            // Never let notification failures break the main business flow.
            eprintln!(
                "notification dispatch error (non-fatal): {:?} for event {} in group {}",
                e, event_name, group_id
            );
        }
    }

    /// Notify a specific user about a group-related event (e.g. an invitee who is not yet a member).
    /// Only user-level preferences are checked because group-level prefs do not apply yet.
    pub async fn notify_user_event(&self, event_name: &str, user_id: UserId, group_id: GroupId) {
        if let Err(e) = self
            .notify_user_event_inner(event_name, user_id, group_id)
            .await
        {
            eprintln!(
                "notification dispatch error (non-fatal): {:?} for event {} to user {} in group {}",
                e, event_name, user_id, group_id
            );
        }
    }

    async fn notify_group_event_inner(
        &self,
        event_name: &str,
        group_id: GroupId,
    ) -> Result<(), RepoError> {
        let group_name = match self.group_repo.get_group_details(group_id)? {
            Some(details) => details.name,
            None => return Ok(()),
        };

        // 1. Discover active members (the service auto-broadcasts per requirement)
        let members = self.group_repo.get_group_members(group_id)?;

        // 2. Resolve event id and channel ids (small tables)
        let events = self.notification_repo.get_events()?;
        let event_id: Option<NotificationEventId> = events
            .into_iter()
            .find(|e| e.name == event_name)
            .map(|e| e.id);

        let Some(event_id) = event_id else {
            return Ok(());
        };

        let channels = self.notification_repo.get_channels()?;
        let email_channel_id = channels.iter().find(|c| c.name == "email").map(|c| c.id);
        let web_channel_id = channels.iter().find(|c| c.name == "web").map(|c| c.id);

        for member in members {
            // Only consider active members
            if member.status
                != crate::infrastructure::db::models::group::GroupMemberStatusModel::Active
            {
                continue;
            }

            let user_id = UserId(member.user_id);

            if let Some(email_channel_id) = email_channel_id {
                if self.is_group_channel_enabled(user_id, group_id, event_id, email_channel_id) {
                    let Ok(recipient) = DomainEmail::new(member.email.clone()) else {
                        continue;
                    };

                    let _ = self
                        .send_email_for_event(event_name, &recipient, &group_name)
                        .await;
                }
            }

            if let Some(web_channel_id) = web_channel_id {
                if self.is_group_channel_enabled(user_id, group_id, event_id, web_channel_id) {
                    if let Err(e) =
                        self.save_web_for_event(event_name, user_id, group_id, &group_name)
                    {
                        eprintln!(
                            "web notification save error (non-fatal): {:?} for event {} to user {} in group {}",
                            e, event_name, user_id, group_id
                        );
                    }
                }
            }
        }

        Ok(())
    }

    async fn notify_user_event_inner(
        &self,
        event_name: &str,
        user_id: UserId,
        group_id: GroupId,
    ) -> Result<(), RepoError> {
        let group_name = match self.group_repo.get_group_details(group_id)? {
            Some(details) => details.name,
            None => return Ok(()),
        };

        let user = match self.user_repo.find_by_id(&user_id)? {
            Some(user) => user,
            None => return Ok(()),
        };

        let events = self.notification_repo.get_events()?;
        let event_id: Option<NotificationEventId> = events
            .into_iter()
            .find(|e| e.name == event_name)
            .map(|e| e.id);

        let Some(event_id) = event_id else {
            return Ok(());
        };

        let channels = self.notification_repo.get_channels()?;
        let email_channel_id = channels.iter().find(|c| c.name == "email").map(|c| c.id);
        let web_channel_id = channels.iter().find(|c| c.name == "web").map(|c| c.id);

        if let Some(email_channel_id) = email_channel_id {
            if self.is_user_channel_enabled(user_id, event_id, email_channel_id) {
                let Ok(recipient) = DomainEmail::new(user.email.clone()) else {
                    return Ok(());
                };

                let _ = self
                    .send_email_for_event(event_name, &recipient, &group_name)
                    .await;
            }
        }

        if let Some(web_channel_id) = web_channel_id {
            if self.is_user_channel_enabled(user_id, event_id, web_channel_id) {
                if let Err(e) = self.save_web_for_event(event_name, user_id, group_id, &group_name)
                {
                    eprintln!(
                        "web notification save error (non-fatal): {:?} for event {} to user {} in group {}",
                        e, event_name, user_id, group_id
                    );
                }
            }
        }

        Ok(())
    }

    fn is_group_channel_enabled(
        &self,
        user_id: UserId,
        group_id: GroupId,
        event_id: NotificationEventId,
        channel_id: NotificationChannelId,
    ) -> bool {
        // For group-scoped events, BOTH the group preference for this (user, group, event, channel)
        // AND the global user preference for (user, event, channel) must be enabled.
        let group_enabled = self
            .notification_repo
            .get_group_preferences(user_id, group_id)
            .map(|prefs| {
                prefs
                    .iter()
                    .any(|p| p.event_id == event_id && p.channel_id == channel_id && p.enabled)
            })
            .unwrap_or(false);

        let user_enabled = self
            .notification_repo
            .get_user_preferences(user_id)
            .map(|prefs| {
                prefs
                    .iter()
                    .any(|p| p.event_id == event_id && p.channel_id == channel_id && p.enabled)
            })
            .unwrap_or(false);

        group_enabled && user_enabled
    }

    fn is_user_channel_enabled(
        &self,
        user_id: UserId,
        event_id: NotificationEventId,
        channel_id: NotificationChannelId,
    ) -> bool {
        self.notification_repo
            .get_user_preferences(user_id)
            .map(|prefs| {
                prefs
                    .iter()
                    .any(|p| p.event_id == event_id && p.channel_id == channel_id && p.enabled)
            })
            .unwrap_or(false)
    }

    fn save_web_for_event(
        &self,
        event_name: &str,
        user_id: UserId,
        group_id: GroupId,
        group_name: &str,
    ) -> Result<(), PersistentNotificationError> {
        match event_name {
            "withdraw_proposal_created" => self
                .persistent_notification_service
                .save_withdraw_proposal_created(user_id, group_id, group_name),
            "proposal_approved" => self
                .persistent_notification_service
                .save_proposal_approved(user_id, group_id, group_name),
            "proposal_rejected" => self
                .persistent_notification_service
                .save_proposal_rejected(user_id, group_id, group_name),
            "proposal_executed" => self
                .persistent_notification_service
                .save_proposal_executed(user_id, group_id, group_name),
            "new_member_added" => self
                .persistent_notification_service
                .save_new_member_added(user_id, group_id, group_name),
            "fund_round_created" => self
                .persistent_notification_service
                .save_fund_round_created(user_id, group_id, group_name),
            "investment_created" => self
                .persistent_notification_service
                .save_investment_created(user_id, group_id, group_name),
            "investment_matured" => self
                .persistent_notification_service
                .save_investment_matured(user_id, group_id, group_name),
            "expense_created" => self
                .persistent_notification_service
                .save_expense_created(user_id, group_id, group_name),
            _ => Ok(()),
        }
    }

    async fn send_email_for_event(
        &self,
        event_name: &str,
        to: &DomainEmail,
        group_name: &str,
    ) -> Result<(), crate::infrastructure::email::email_sender::EmailServiceError> {
        match event_name {
            "withdraw_proposal_created" => {
                self.email_service
                    .send_withdraw_proposal_created_email(to, group_name)
                    .await
            }
            "proposal_approved" => {
                self.email_service
                    .send_proposal_approved_email(to, group_name)
                    .await
            }
            "proposal_rejected" => {
                self.email_service
                    .send_proposal_rejected_email(to, group_name)
                    .await
            }
            "proposal_executed" => {
                self.email_service
                    .send_proposal_executed_email(to, group_name)
                    .await
            }

            "new_member_added" => {
                self.email_service
                    .send_new_member_added_email(to, group_name)
                    .await
            }

            "fund_round_created" => {
                self.email_service
                    .send_fund_round_created_email(to, group_name)
                    .await
            }
            "investment_created" => {
                self.email_service
                    .send_investment_created_email(to, group_name)
                    .await
            }
            "investment_matured" => {
                self.email_service
                    .send_investment_matured_email(to, group_name)
                    .await
            }
            "expense_created" => {
                self.email_service
                    .send_expense_created_email(to, group_name)
                    .await
            }

            _ => Ok(()),
        }
    }
}
