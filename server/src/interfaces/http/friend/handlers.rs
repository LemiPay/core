use axum::{
    Json,
    extract::{Path, Query, State},
};
use uuid::Uuid;

use crate::interfaces::http::{
    auth::extractor::AuthUser,
    error::AppError,
    friend::dto::{FriendResponse, RespondRequest, SearchQuery, UserSearchResponse},
};
use crate::setup::state::SharedState;

pub async fn send_request(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(addressee_id): Path<Uuid>,
) -> Result<Json<FriendResponse>, AppError> {
    let details = state
        .friend_service
        .send_request(user.user_id.0, addressee_id)
        .map_err(AppError::from)?;
    Ok(Json(FriendResponse::from_details(details)))
}

pub async fn respond_request(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(requester_id): Path<Uuid>,
    Json(payload): Json<RespondRequest>,
) -> Result<Json<FriendResponse>, AppError> {
    let details = state
        .friend_service
        .respond_request(user.user_id.0, requester_id, &payload.action)
        .map_err(AppError::from)?;
    Ok(Json(FriendResponse::from_details(details)))
}

pub async fn list_friends(
    State(state): State<SharedState>,
    user: AuthUser,
) -> Result<Json<Vec<FriendResponse>>, AppError> {
    let friends = state
        .friend_service
        .list_friends(user.user_id.0)
        .map_err(AppError::from)?;
    Ok(Json(
        friends
            .into_iter()
            .map(FriendResponse::from_details)
            .collect(),
    ))
}

pub async fn list_received_requests(
    State(state): State<SharedState>,
    user: AuthUser,
) -> Result<Json<Vec<FriendResponse>>, AppError> {
    let requests = state
        .friend_service
        .list_received_requests(user.user_id.0)
        .map_err(AppError::from)?;
    Ok(Json(
        requests
            .into_iter()
            .map(FriendResponse::from_details)
            .collect(),
    ))
}

pub async fn list_sent_requests(
    State(state): State<SharedState>,
    user: AuthUser,
) -> Result<Json<Vec<FriendResponse>>, AppError> {
    let requests = state
        .friend_service
        .list_sent_requests(user.user_id.0)
        .map_err(AppError::from)?;
    Ok(Json(
        requests
            .into_iter()
            .map(FriendResponse::from_details)
            .collect(),
    ))
}

pub async fn unfriend(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(other_id): Path<Uuid>,
) -> Result<Json<()>, AppError> {
    state
        .friend_service
        .unfriend(user.user_id.0, other_id)
        .map_err(AppError::from)?;
    Ok(Json(()))
}

pub async fn search_users(
    State(state): State<SharedState>,
    user: AuthUser,
    Query(params): Query<SearchQuery>,
) -> Result<Json<Vec<UserSearchResponse>>, AppError> {
    let query = params.q.unwrap_or_default();
    if query.trim().is_empty() {
        return Ok(Json(vec![]));
    }
    let results = state
        .friend_service
        .search_users(user.user_id.0, &query)
        .map_err(AppError::from)?;
    Ok(Json(results.into_iter().map(Into::into).collect()))
}

pub async fn block_user(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(blocked_id): Path<Uuid>,
) -> Result<Json<()>, AppError> {
    state
        .friend_service
        .block_user(user.user_id.0, blocked_id)
        .map_err(AppError::from)?;
    Ok(Json(()))
}
