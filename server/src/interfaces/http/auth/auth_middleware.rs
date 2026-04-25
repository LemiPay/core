use crate::setup::state::AppState;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn auth_middleware(mut req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    // 1. header
    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let auth_str = auth_header.to_str().map_err(|_| StatusCode::UNAUTHORIZED)?;

    let token = auth_str
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // 2. obtener state
    let state = req
        .extensions()
        .get::<AppState>()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    // 3. verificar token
    let user_id = state
        .login_use_case
        .token_service
        .verify(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // 4. insertar en extensions
    req.extensions_mut().insert(user_id.0);

    // 5. continuar
    Ok(next.run(req).await)
}
