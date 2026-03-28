use crate::security::jwt::decode_jwt;
use axum::body::Body;
use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

///
/// Validates request JWT
///
pub async fn auth_middleware(mut req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    // 1. Obtener header
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    // 2. Validar formato
    let token = match auth_header {
        Some(h) if h.starts_with("Bearer ") => &h[7..],
        _ => return Err(StatusCode::UNAUTHORIZED),
    };

    // 3. Decodificar JWT
    let claims = decode_jwt(token).map_err(|_| StatusCode::UNAUTHORIZED)?;

    // 5. Guardar user_id en request extensions
    req.extensions_mut().insert(claims.sub);

    // 6. Continuar request
    Ok(next.run(req).await)
}
