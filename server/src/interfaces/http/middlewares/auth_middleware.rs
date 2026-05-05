use crate::setup::state::SharedState;

use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn auth_middleware(
    State(state): State<SharedState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // header
    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let auth_str = auth_header.to_str().map_err(|_| StatusCode::UNAUTHORIZED)?;

    let token = auth_str
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // verificar token
    let user_id = state
        .auth_service
        .login
        .token_service
        .verify(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // insertar en extensions
    req.extensions_mut().insert(user_id.0);

    // 5. continuar
    Ok(next.run(req).await)
}
