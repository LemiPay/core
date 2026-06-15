use chrono::NaiveDateTime;

use crate::domain::group::GroupId;
use crate::domain::notification::types::{
    NotificationChannelId, NotificationEventId, NotificationRecordId,
};
use crate::domain::user::UserId;

#[derive(Clone)]
pub struct NotificationChannel {
    pub id: NotificationChannelId,
    pub name: String,
}

#[derive(Clone)]
pub struct NotificationEvent {
    pub id: NotificationEventId,
    pub name: String,
}

#[derive(Clone)]
pub struct UserNotificationPreference {
    pub user_id: UserId,
    pub event_id: NotificationEventId,
    pub channel_id: NotificationChannelId,
    pub enabled: bool,
}

#[derive(Clone)]
pub struct GroupNotificationPreference {
    pub user_id: UserId,
    pub group_id: GroupId,
    pub event_id: NotificationEventId,
    pub channel_id: NotificationChannelId,
    pub enabled: bool,
}

#[derive(Clone)]
pub struct NotificationRecord {
    pub id: NotificationRecordId,
    pub user_id: UserId,
    pub group_id: Option<GroupId>,
    pub event_name: String,
    pub group_name: Option<String>,
    pub read: bool,
    pub created_at: NaiveDateTime,
}
