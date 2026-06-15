use std::sync::Arc;

use crate::application::common::repo_error::RepoError;
use crate::application::group::traits::repository::GroupRepository;
use crate::application::notifications::repository::NotificationRepository;
use crate::application::users::traits::repository::UserRepository;
use crate::domain::group::GroupId;
use crate::domain::notification::types::NotificationEventId;
use crate::domain::user::Email as DomainEmail;
use crate::domain::user::UserId;
use crate::infrastructure::email::email_sender::EmailService;

/// Central notification dispatcher (the "observer").
/// Currently focused on the email channel per product decision.
/// Web notifications for proposals are handled client-side via polling.
#[derive(Clone)]
pub struct NotificationService {
    pub notification_repo: Arc<dyn NotificationRepository>,
    pub email_service: Arc<dyn EmailService>,
    pub group_repo: Arc<dyn GroupRepository>,
    pub user_repo: Arc<dyn UserRepository>,
}

impl NotificationService {
    /// Notify about a group-scoped event (the main entry point for most business events).
    /// The service expands to active group members, checks their prefs (group then user level)
    /// for the email channel, and if enabled calls the matching specific email method.
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

        // 2. Resolve event id and email channel id (small tables)
        let events = self.notification_repo.get_events()?;
        let event_id: Option<NotificationEventId> = events
            .into_iter()
            .find(|e| e.name == event_name)
            .map(|e| e.id);

        let Some(event_id) = event_id else {
            return Ok(());
        };

        let channels = self.notification_repo.get_channels()?;
        let email_channel_id = channels
            .into_iter()
            .find(|c| c.name == "email")
            .map(|c| c.id);

        let Some(email_channel_id) = email_channel_id else {
            return Ok(());
        };

        for member in members {
            // Only consider active members
            if member.status
                != crate::infrastructure::db::models::group::GroupMemberStatusModel::Active
            {
                continue;
            }

            let user_id = UserId(member.user_id);

            // 3. Preference rule (per user requirement):
            // For group-scoped events, BOTH the group preference for this (user, group, event, 'email')
            // AND the global user preference for (user, event, 'email') must be enabled.
            // If either is missing or disabled, we do not send.
            let group_enabled = self
                .notification_repo
                .get_group_preferences(user_id, group_id)
                .map(|prefs| {
                    prefs.iter().any(|p| {
                        p.event_id == event_id && p.channel_id == email_channel_id && p.enabled
                    })
                })
                .unwrap_or(false);

            let user_enabled = self
                .notification_repo
                .get_user_preferences(user_id)
                .map(|prefs| {
                    prefs.iter().any(|p| {
                        p.event_id == event_id && p.channel_id == email_channel_id && p.enabled
                    })
                })
                .unwrap_or(false);

            if !(group_enabled && user_enabled) {
                continue;
            }

            // 4. We have the email in the member details in the current schema
            let recipient = match member.email.parse() {
                Ok(addr) => DomainEmail(addr),
                Err(_) => continue,
            };

            // 5. Fire the specific email method (non-blocking for the caller in practice)
            let _ = self
                .send_for_event(event_name, &recipient, &group_name)
                .await;
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
        let email_channel_id = channels
            .into_iter()
            .find(|c| c.name == "email")
            .map(|c| c.id);

        let Some(email_channel_id) = email_channel_id else {
            return Ok(());
        };

        let user_enabled = self
            .notification_repo
            .get_user_preferences(user_id)
            .map(|prefs| {
                prefs.iter().any(|p| {
                    p.event_id == event_id && p.channel_id == email_channel_id && p.enabled
                })
            })
            .unwrap_or(false);

        if !user_enabled {
            return Ok(());
        }

        let recipient = match user.email.parse() {
            Ok(addr) => DomainEmail(addr),
            Err(_) => return Ok(()),
        };

        let _ = self
            .send_for_event(event_name, &recipient, &group_name)
            .await;

        Ok(())
    }

    async fn send_for_event(
        &self,
        event_name: &str,
        to: &DomainEmail,
        group_name: &str,
    ) -> Result<(), crate::infrastructure::email::email_sender::EmailServiceError> {
        match event_name {
            "proposal_created" => {
                self.email_service
                    .send_proposal_created_email(to, group_name)
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
