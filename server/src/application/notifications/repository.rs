use crate::application::common::repo_error::RepoError;

use crate::domain::{
    group::GroupId,
    notification::{
        GroupNotificationPreference, NotificationChannel, NotificationEvent,
        UserNotificationPreference,
    },
    user::UserId,
};

pub trait NotificationRepository: Send + Sync {
    // =========================
    // Metadata
    // =========================

    fn get_events(&self) -> Result<Vec<NotificationEvent>, RepoError>;

    fn get_channels(&self) -> Result<Vec<NotificationChannel>, RepoError>;

    // =========================
    // User preferences
    // =========================

    fn get_user_preferences(
        &self,
        user_id: UserId,
    ) -> Result<Vec<UserNotificationPreference>, RepoError>;

    fn upsert_user_preference(
        &self,
        preference: UserNotificationPreference,
    ) -> Result<(), RepoError>;

    // =========================
    // Group preferences
    // =========================

    fn get_group_preferences(
        &self,
        user_id: UserId,
        group_id: GroupId,
    ) -> Result<Vec<GroupNotificationPreference>, RepoError>;

    fn upsert_group_preference(
        &self,
        preference: GroupNotificationPreference,
    ) -> Result<(), RepoError>;
}
