use crate::application::notifications::error::NotificationError;
use crate::domain::notification::{
    GroupNotificationPreference, NotificationChannel, NotificationEvent, UserNotificationPreference,
};
use async_trait::async_trait;
use uuid::Uuid;

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
        user_id: Uuid,
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
        user_id: Uuid,
        group_id: Uuid,
    ) -> Result<Vec<GroupNotificationPreference>, NotificationError>;

    async fn upsert_group_preference(
        &self,
        preference: GroupNotificationPreference,
    ) -> Result<(), NotificationError>;
}
