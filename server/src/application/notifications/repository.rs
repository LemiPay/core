use crate::application::common::repo_error::RepoError;

use crate::domain::{
    group::GroupId,
    notification::types::NotificationRecordId,
    notification::{
        GroupNotificationPreference, NotificationChannel, NotificationEvent, NotificationRecord,
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

    // =========================
    // Default initialization (create explicit rows with enabled=true)
    // =========================

    /// Create default (enabled=true) user_notification_preference rows for every event+channel combo for this user.
    /// Safe to call multiple times (uses upsert).
    fn initialize_defaults_for_user(&self, user_id: UserId) -> Result<(), RepoError>;

    /// Create default (enabled=true) group_notification_preference rows for every event+channel combo
    /// for this (user, group). Safe to call multiple times.
    fn initialize_defaults_for_user_in_group(
        &self,
        user_id: UserId,
        group_id: GroupId,
    ) -> Result<(), RepoError>;

    // =========================
    // Persistent web notifications
    // =========================

    fn list_user_notifications(
        &self,
        user_id: UserId,
        read_filter: Option<bool>,
        limit: Option<i64>,
    ) -> Result<Vec<NotificationRecord>, RepoError>;

    fn mark_notification_read(
        &self,
        user_id: UserId,
        notification_id: NotificationRecordId,
    ) -> Result<bool, RepoError>;

    fn mark_all_notifications_read(&self, user_id: UserId) -> Result<u64, RepoError>;
}
