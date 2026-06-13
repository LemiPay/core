use diesel::prelude::*;
use uuid::Uuid;

use crate::infrastructure::db::schema;

// =========================
// Notification Events
// =========================

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = schema::notification_event)]
pub struct NotificationEvent {
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
pub struct NotificationChannel {
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
pub struct UserNotificationPreference {
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
pub struct GroupNotificationPreference {
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
