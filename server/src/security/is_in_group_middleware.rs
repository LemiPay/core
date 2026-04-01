use crate::data::state::SharedState;
use crate::helpers::validations::{is_group_active, is_member};
use crate::schema::vote::user_id;
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
    if !is_group_active(group_id, group_repo.clone()).unwrap_or(false) {
        return Err(StatusCode::FORBIDDEN);
    }

    match is_member(user.user_id, group_id, group_repo) {
        Ok(true) => Ok(next.run(req).await),
        Ok(false) => Err(StatusCode::FORBIDDEN),
        Err(_) => Err(StatusCode::BAD_REQUEST),
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
    if !is_group_active(group_id, group_repo.clone()).unwrap_or(false) {
        return Err(StatusCode::FORBIDDEN);
    }
    match group_repo.is_admin(user.user_id, group_id) {
        Ok(true) => Ok(next.run(req).await),
        Ok(false) => Err(StatusCode::FORBIDDEN),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}
