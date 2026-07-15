use uuid::Uuid;

use crate::application::{
    common::repo_error::RepoError,
    friend::dto::{FriendDetails, NewFriend, UserSearchResult},
};
use crate::domain::friend::FriendStatus;

pub trait FriendRepository: Send + Sync {
    fn find_relationship(
        &self,
        user_id_1: Uuid,
        user_id_2: Uuid,
    ) -> Result<Option<(FriendStatus, Uuid, Uuid)>, RepoError>;

    fn find_relationship_bidirectional(
        &self,
        user_id_1: Uuid,
        user_id_2: Uuid,
    ) -> Result<Option<(FriendStatus, Uuid, Uuid)>, RepoError>;

    fn insert(&self, new_friend: NewFriend) -> Result<(), RepoError>;

    fn update_status(
        &self,
        requester_id: Uuid,
        addressee_id: Uuid,
        status: FriendStatus,
    ) -> Result<(), RepoError>;

    fn find_with_details(
        &self,
        requester_id: Uuid,
        addressee_id: Uuid,
    ) -> Result<Option<FriendDetails>, RepoError>;

    fn list_by_user_and_status(
        &self,
        user_id: Uuid,
        status: FriendStatus,
    ) -> Result<Vec<FriendDetails>, RepoError>;

    fn list_friends(&self, user_id: Uuid) -> Result<Vec<FriendDetails>, RepoError>;

    fn list_received_requests(&self, user_id: Uuid) -> Result<Vec<FriendDetails>, RepoError>;

    fn list_sent_requests(&self, user_id: Uuid) -> Result<Vec<FriendDetails>, RepoError>;

    fn delete(&self, requester_id: Uuid, addressee_id: Uuid) -> Result<(), RepoError>;

    fn search_users(
        &self,
        current_user_id: Uuid,
        query: &str,
    ) -> Result<Vec<UserSearchResult>, RepoError>;
}
