use serde::{Deserialize, Serialize};

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
