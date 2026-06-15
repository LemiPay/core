use crate::domain::group::GroupId;
use crate::domain::user::UserId;

#[derive(Debug)]
pub enum PersistentNotificationError {
    SaveFailed,
    Internal,
}

pub trait PersistentNotificationService: Send + Sync {
    fn example(&self, user_id: UserId) -> Result<(), PersistentNotificationError>;

    fn save_welcome_notification(
        &self,
        user_id: UserId,
        name: &str,
    ) -> Result<(), PersistentNotificationError>;

    fn save_login_alert(
        &self,
        user_id: UserId,
        name: &str,
    ) -> Result<(), PersistentNotificationError>;

    // Business event notifications
    fn save_withdraw_proposal_created(
        &self,
        user_id: UserId,
        group_id: GroupId,
        group_name: &str,
    ) -> Result<(), PersistentNotificationError>;

    fn save_proposal_approved(
        &self,
        user_id: UserId,
        group_id: GroupId,
        group_name: &str,
    ) -> Result<(), PersistentNotificationError>;

    fn save_proposal_rejected(
        &self,
        user_id: UserId,
        group_id: GroupId,
        group_name: &str,
    ) -> Result<(), PersistentNotificationError>;

    fn save_proposal_executed(
        &self,
        user_id: UserId,
        group_id: GroupId,
        group_name: &str,
    ) -> Result<(), PersistentNotificationError>;

    fn save_new_member_added(
        &self,
        user_id: UserId,
        group_id: GroupId,
        group_name: &str,
    ) -> Result<(), PersistentNotificationError>;

    fn save_fund_round_created(
        &self,
        user_id: UserId,
        group_id: GroupId,
        group_name: &str,
    ) -> Result<(), PersistentNotificationError>;

    fn save_investment_created(
        &self,
        user_id: UserId,
        group_id: GroupId,
        group_name: &str,
    ) -> Result<(), PersistentNotificationError>;

    fn save_investment_matured(
        &self,
        user_id: UserId,
        group_id: GroupId,
        group_name: &str,
    ) -> Result<(), PersistentNotificationError>;

    fn save_expense_created(
        &self,
        user_id: UserId,
        group_id: GroupId,
        group_name: &str,
    ) -> Result<(), PersistentNotificationError>;
}
