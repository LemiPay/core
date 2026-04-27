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
    match state.group_service.is_member(user.user_id, group_id) {
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
    match state.group_service.is_admin(user.user_id, group_id) {
        Ok(true) => Ok(next.run(req).await),
        Ok(false) => Err(StatusCode::FORBIDDEN),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Variante de `is_group_admin_middleware` para rutas con dos path params
/// `{group_id}/{resource_id}` (por ejemplo, `/admin/{group_id}/{expense_id}`).
/// Sólo el `group_id` participa en el chequeo de admin; el `resource_id`
/// se descarta aquí y queda disponible para el handler.
pub async fn is_group_admin_for_resource_middleware(
    State(state): State<SharedState>,
    Path((group_id, _resource_id)): Path<(Uuid, Uuid)>,
    user: AuthUser,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    match state.group_service.is_admin(user.user_id, group_id) {
        Ok(true) => Ok(next.run(req).await),
        Ok(false) => Err(StatusCode::FORBIDDEN),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
