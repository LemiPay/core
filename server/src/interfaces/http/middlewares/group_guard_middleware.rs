use axum::{
    body::Body,
    extract::{Path, State},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::{
    domain::group::GroupId,
    interfaces::http::auth::extractor::AuthUser,
    setup::state::SharedState,
};

pub async fn is_in_group_middleware(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
    user: AuthUser,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    match state
        .group_service
        .get_group_members
        .group_repo
        .is_member(user.user_id, GroupId(group_id))
    {
        Ok(true) => Ok(next.run(req).await),
        Ok(false) => Err(StatusCode::FORBIDDEN),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn is_group_admin_middleware(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
    user: AuthUser,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    match state
        .group_service
        .get_group_members
        .group_repo
        .is_admin(user.user_id, GroupId(group_id))
    {
        Ok(true) => Ok(next.run(req).await),
        Ok(false) => Err(StatusCode::FORBIDDEN),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
