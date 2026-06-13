use async_trait::async_trait;

use crate::application::notifications::error::NotificationError;

use crate::domain::{
    group::GroupId,
    notification::{
        GroupNotificationPreference, NotificationChannel, NotificationEvent,
        UserNotificationPreference,
    },
    user::UserId,
};

#[async_trait]
pub trait NotificationRepository: Send + Sync {
    // =========================
    // Metadata
    // =========================

    async fn get_events(&self) -> Result<Vec<NotificationEvent>, NotificationError>;

    async fn get_channels(&self) -> Result<Vec<NotificationChannel>, NotificationError>;

    // =========================
    // User preferences
    // =========================

    async fn get_user_preferences(
        &self,
        user_id: UserId,
    ) -> Result<Vec<UserNotificationPreference>, NotificationError>;

    async fn upsert_user_preference(
        &self,
        preference: UserNotificationPreference,
    ) -> Result<(), NotificationError>;

    // =========================
    // Group preferences
    // =========================

    async fn get_group_preferences(
        &self,
        user_id: UserId,
        group_id: GroupId,
    ) -> Result<Vec<GroupNotificationPreference>, NotificationError>;

    async fn upsert_group_preference(
        &self,
        preference: GroupNotificationPreference,
    ) -> Result<(), NotificationError>;
}
