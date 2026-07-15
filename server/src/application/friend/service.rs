use std::sync::Arc;

use uuid::Uuid;

use crate::application::friend::{
    dto::{FriendDetails, NewFriend, UserSearchResult},
    error::FriendError,
    traits::repository::FriendRepository,
};
use crate::domain::friend::FriendStatus;

#[derive(Clone)]
pub struct FriendService {
    pub friend_repo: Arc<dyn FriendRepository>,
}

impl FriendService {
    pub fn send_request(
        &self,
        requester_id: Uuid,
        addressee_id: Uuid,
    ) -> Result<FriendDetails, FriendError> {
        if requester_id == addressee_id {
            return Err(FriendError::SameUser);
        }

        let existing = self
            .friend_repo
            .find_relationship_bidirectional(requester_id, addressee_id)?;

        if let Some((status, _r, _a)) = existing {
            match status {
                FriendStatus::Accepted => return Err(FriendError::AlreadyFriends),
                FriendStatus::Pending => return Err(FriendError::PendingRequestExists),
                FriendStatus::Blocked => return Err(FriendError::CannotRequestBlocked),
                FriendStatus::Rejected => {
                    let (stored_r, stored_a) = (_r, _a);
                    if stored_r == requester_id && stored_a == addressee_id {
                        return Err(FriendError::PendingRequestExists);
                    }
                }
            }
        }

        self.friend_repo.insert(NewFriend {
            requester_id,
            addressee_id,
            status: FriendStatus::Pending,
        })?;

        self.friend_repo
            .find_with_details(requester_id, addressee_id)?
            .ok_or(FriendError::Internal)
    }

    pub fn respond_request(
        &self,
        user_id: Uuid,
        requester_id: Uuid,
        action: &str,
    ) -> Result<FriendDetails, FriendError> {
        let existing = self.friend_repo.find_relationship(requester_id, user_id)?;

        let (status, _r, _a) = existing.ok_or(FriendError::NotFound)?;

        if status != FriendStatus::Pending {
            return Err(FriendError::NotFound);
        }

        let new_status = match action {
            "accept" => FriendStatus::Accepted,
            "reject" => FriendStatus::Rejected,
            _ => return Err(FriendError::InvalidAction),
        };

        self.friend_repo
            .update_status(requester_id, user_id, new_status)?;

        self.friend_repo
            .find_with_details(requester_id, user_id)?
            .ok_or(FriendError::Internal)
    }

    pub fn list_friends(&self, user_id: Uuid) -> Result<Vec<FriendDetails>, FriendError> {
        self.friend_repo
            .list_friends(user_id)
            .map_err(FriendError::from)
    }

    pub fn list_received_requests(&self, user_id: Uuid) -> Result<Vec<FriendDetails>, FriendError> {
        self.friend_repo
            .list_received_requests(user_id)
            .map_err(FriendError::from)
    }

    pub fn list_sent_requests(&self, user_id: Uuid) -> Result<Vec<FriendDetails>, FriendError> {
        self.friend_repo
            .list_sent_requests(user_id)
            .map_err(FriendError::from)
    }

    pub fn unfriend(&self, user_id: Uuid, other_id: Uuid) -> Result<(), FriendError> {
        let existing = self
            .friend_repo
            .find_relationship_bidirectional(user_id, other_id)?;

        let (status, requester_id, addressee_id) = existing.ok_or(FriendError::NotFound)?;

        match status {
            FriendStatus::Pending | FriendStatus::Accepted | FriendStatus::Rejected => {
                self.friend_repo.delete(requester_id, addressee_id)?;
                Ok(())
            }
            FriendStatus::Blocked => Err(FriendError::NotFound),
        }
    }

    pub fn search_users(
        &self,
        user_id: Uuid,
        query: &str,
    ) -> Result<Vec<UserSearchResult>, FriendError> {
        self.friend_repo
            .search_users(user_id, query)
            .map_err(FriendError::from)
    }

    pub fn block_user(&self, user_id: Uuid, blocked_id: Uuid) -> Result<(), FriendError> {
        if user_id == blocked_id {
            return Err(FriendError::SameUser);
        }

        let existing = self
            .friend_repo
            .find_relationship_bidirectional(user_id, blocked_id)?;

        if let Some((_status, requester_id, addressee_id)) = existing {
            if requester_id == user_id || addressee_id == user_id {
                let (r, a) = if requester_id == user_id {
                    (requester_id, addressee_id)
                } else {
                    (addressee_id, requester_id)
                };
                self.friend_repo
                    .update_status(r, a, FriendStatus::Blocked)?;
            }
        } else {
            self.friend_repo.insert(NewFriend {
                requester_id: user_id,
                addressee_id: blocked_id,
                status: FriendStatus::Blocked,
            })?;
        }

        Ok(())
    }
}
