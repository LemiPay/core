use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ListNotificationsQuery {
    pub read: Option<bool>,
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpsertPreferenceRequest {
    pub event_id: String,
    pub channel_id: String,
    pub enabled: bool,
}

#[derive(Serialize)]
pub struct EventResponse {
    pub id: String,
    pub name: String,
}

#[derive(Serialize)]
pub struct ChannelResponse {
    pub id: String,
    pub name: String,
}

#[derive(Serialize)]
pub struct PreferenceResponse {
    pub user_id: String,
    pub event_id: String,
    pub channel_id: String,
    pub enabled: bool,
}

#[derive(Serialize)]
pub struct GroupPreferenceResponse {
    pub user_id: String,
    pub group_id: String,
    pub event_id: String,
    pub channel_id: String,
    pub enabled: bool,
}

#[derive(Serialize)]
pub struct NotificationRecordResponse {
    pub id: String,
    pub event_name: String,
    pub group_id: Option<String>,
    pub group_name: Option<String>,
    pub read: bool,
    pub created_at: String,
}
