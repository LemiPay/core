use std::sync::Arc;

use crate::application::friend::FriendService;
use crate::infrastructure::db::repositories::friend_repo_impl::DieselFriendRepository;

pub fn build_friend_service(friend_repo: Arc<DieselFriendRepository>) -> FriendService {
    FriendService { friend_repo }
}
