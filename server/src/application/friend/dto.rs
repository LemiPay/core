use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::domain::friend::FriendStatus;

pub struct FriendDetails {
    pub user_id: Uuid,
    pub name: String,
    pub email: String,
    pub status: FriendStatus,
    pub created_at: NaiveDateTime,
}

pub struct NewFriend {
    pub requester_id: Uuid,
    pub addressee_id: Uuid,
    pub status: FriendStatus,
}

pub struct UserSearchResult {
    pub user_id: Uuid,
    pub name: String,
    pub email: String,
    pub is_friend: bool,
}
