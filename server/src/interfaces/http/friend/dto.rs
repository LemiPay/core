use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct FriendResponse {
    pub user_id: Uuid,
    pub name: String,
    pub email: String,
    pub status: String,
    pub created_at: String,
}

impl FriendResponse {
    pub fn from_details(details: crate::application::friend::dto::FriendDetails) -> Self {
        Self {
            user_id: details.user_id,
            name: details.name,
            email: details.email,
            status: details.status.as_str().to_string(),
            created_at: details.created_at.to_string(),
        }
    }
}

#[derive(Deserialize)]
pub struct RespondRequest {
    pub action: String,
}

#[derive(Serialize)]
pub struct UserSearchResponse {
    pub user_id: Uuid,
    pub name: String,
    pub email: String,
    pub is_friend: bool,
}

impl From<crate::application::friend::dto::UserSearchResult> for UserSearchResponse {
    fn from(value: crate::application::friend::dto::UserSearchResult) -> Self {
        Self {
            user_id: value.user_id,
            name: value.name,
            email: value.email,
            is_friend: value.is_friend,
        }
    }
}

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
}
