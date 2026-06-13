use crate::domain::group::GroupId;
use crate::domain::notification::types::{NotificationChannelId, NotificationEventId};
use crate::domain::user::UserId;

pub struct NotificationChannel {
    id: NotificationChannelId,
    name: String,
}

pub struct NotificationEvent {
    id: NotificationEventId,
    name: String,
}

pub struct UserNotificationPreference {
    user_id: UserId,
    event_id: NotificationEventId,
    channel_id: NotificationChannelId,
    enabled: bool,
}

pub struct GroupNotificationPreference {
    user_id: UserId,
    group_id: GroupId,
    event_id: NotificationEventId,
    channel_id: NotificationChannelId,
    enabled: bool,
}
