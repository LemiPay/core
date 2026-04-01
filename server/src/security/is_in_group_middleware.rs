use crate::data::state::SharedState;
use crate::security::auth_extractor::AuthUser;
use axum::body::Body;
use axum::{
    extract::{Path, State},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

pub async fn is_in_group_middleware(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
    user: AuthUser,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let group_repo = state.group_service.get_group_repo();

    let is_active = group_repo
        .is_group_active(group_id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !is_active {
        return Err(StatusCode::FORBIDDEN);
    }
    match group_repo.is_member(user.user_id, group_id) {
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
    let group_repo = state.group_service.get_group_repo();
    let is_active = group_repo
        .is_group_active(group_id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if !is_active {
        return Err(StatusCode::FORBIDDEN);
    }
    match group_repo.is_admin(user.user_id, group_id) {
        Ok(true) => Ok(next.run(req).await),
        Ok(false) => Err(StatusCode::FORBIDDEN),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
