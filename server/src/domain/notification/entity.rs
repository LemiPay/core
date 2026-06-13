use crate::domain::group::GroupId;
use crate::domain::notification::types::{NotificationChannelId, NotificationEventId};
use crate::domain::user::UserId;

pub struct NotificationChannel {
    pub id: NotificationChannelId,
    pub name: String,
}

pub struct NotificationEvent {
    pub id: NotificationEventId,
    pub name: String,
}

pub struct UserNotificationPreference {
    pub user_id: UserId,
    pub event_id: NotificationEventId,
    pub channel_id: NotificationChannelId,
    pub enabled: bool,
}

pub struct GroupNotificationPreference {
    pub user_id: UserId,
    pub group_id: GroupId,
    pub event_id: NotificationEventId,
    pub channel_id: NotificationChannelId,
    pub enabled: bool,
}
