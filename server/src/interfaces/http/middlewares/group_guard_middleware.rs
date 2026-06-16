use axum::{
    body::Body,
    extract::{Path, State},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::{
    domain::group::GroupId, interfaces::http::auth::extractor::AuthUser, setup::state::SharedState,
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
        Ok(false) => {
            let body = serde_json::json!({
                "message": "Solo los administradores pueden realizar esta acción"
            });
            let json = serde_json::to_string(&body).unwrap();
            let res = Response::builder()
                .status(StatusCode::FORBIDDEN)
                .header("content-type", "application/json")
                .body(Body::from(json))
                .unwrap();
            Ok(res)
        }
        Err(_) => {
            let body = serde_json::json!({
                "message": "Error interno del servidor"
            });
            let json = serde_json::to_string(&body).unwrap();
            let res = Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("content-type", "application/json")
                .body(Body::from(json))
                .unwrap();
            Ok(res)
        }
    }
}

pub async fn is_group_admin_for_resource_middleware(
    State(state): State<SharedState>,
    Path((group_id, _resource_id)): Path<(Uuid, Uuid)>,
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
