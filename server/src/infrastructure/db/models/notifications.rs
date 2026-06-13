use crate::domain::notification::types::{NotificationChannelId, NotificationEventId};
use crate::domain::notification::{
    GroupNotificationPreference, NotificationChannel, NotificationEvent, UserNotificationPreference,
};
use crate::infrastructure::db::schema;
use diesel::prelude::*;
use uuid::Uuid;

// =========================
// Notification Events
// =========================

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::notification_event)]
pub struct NotificationEventModel {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = schema::notification_event)]
pub struct NewNotificationEvent<'a> {
    pub name: &'a str,
}

// =========================
// Notification Channels
// =========================

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::notification_channel)]
pub struct NotificationChannelModel {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = schema::notification_channel)]
pub struct NewNotificationChannel<'a> {
    pub name: &'a str,
}

// =========================
// User Notification Preferences
// =========================

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(
    table_name = schema::user_notification_preference,
    primary_key(user_id, event_id, channel_id),
)]
pub struct UserNotificationPreferenceModel {
    pub user_id: Uuid,
    pub event_id: Uuid,
    pub channel_id: Uuid,
    pub enabled: bool,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = schema::user_notification_preference)]
pub struct NewUserNotificationPreference {
    pub user_id: Uuid,
    pub event_id: Uuid,
    pub channel_id: Uuid,
    pub enabled: bool,
}

// =========================
// Group Notification Preferences
// =========================

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(
    table_name = schema::group_notification_preference,
    primary_key(user_id, group_id, event_id, channel_id),
)]
pub struct GroupNotificationPreferenceModel {
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub event_id: Uuid,
    pub channel_id: Uuid,
    pub enabled: bool,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = schema::group_notification_preference)]
pub struct NewGroupNotificationPreference {
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub event_id: Uuid,
    pub channel_id: Uuid,
    pub enabled: bool,
}

// impl From<A> for B means A to B using B::from(a) or a.into()

impl From<NotificationEventModel> for NotificationEvent {
    fn from(value: NotificationEventModel) -> Self {
        Self {
            id: NotificationEventId::from(value.id),
            name: value.name,
        }
    }
}

impl From<NotificationChannelModel> for NotificationChannel {
    fn from(value: NotificationChannelModel) -> Self {
        Self {
            id: NotificationChannelId::from(value.id),
            name: value.name,
        }
    }
}

impl From<UserNotificationPreferenceModel> for UserNotificationPreference {
    fn from(value: UserNotificationPreferenceModel) -> Self {
        Self {
            user_id: value.user_id.into(),
            event_id: value.event_id.into(),
            channel_id: value.channel_id.into(),
            enabled: value.enabled,
        }
    }
}

impl From<UserNotificationPreference> for UserNotificationPreferenceModel {
    fn from(value: UserNotificationPreference) -> Self {
        Self {
            user_id: value.user_id.into(),
            event_id: value.event_id.into(),
            channel_id: value.channel_id.into(),
            enabled: value.enabled,
        }
    }
}

impl From<UserNotificationPreference> for NewUserNotificationPreference {
    fn from(value: UserNotificationPreference) -> Self {
        Self {
            user_id: value.user_id.into(),
            event_id: value.event_id.into(),
            channel_id: value.channel_id.into(),
            enabled: value.enabled,
        }
    }
}

impl From<GroupNotificationPreference> for NewGroupNotificationPreference {
    fn from(value: GroupNotificationPreference) -> Self {
        Self {
            user_id: value.user_id.into(),
            group_id: value.group_id.into(),
            event_id: value.event_id.into(),
            channel_id: value.channel_id.into(),
            enabled: value.enabled,
        }
    }
}

impl From<GroupNotificationPreference> for GroupNotificationPreferenceModel {
    fn from(value: GroupNotificationPreference) -> Self {
        Self {
            user_id: value.user_id.into(),
            group_id: value.group_id.into(),
            event_id: value.event_id.into(),
            channel_id: value.channel_id.into(),
            enabled: value.enabled,
        }
    }
}

impl From<GroupNotificationPreferenceModel> for GroupNotificationPreference {
    fn from(value: GroupNotificationPreferenceModel) -> Self {
        Self {
            user_id: value.user_id.into(),
            group_id: value.group_id.into(),
            event_id: value.event_id.into(),
            channel_id: value.channel_id.into(),
            enabled: value.enabled,
        }
    }
}
