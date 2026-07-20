use axum::Json;

use crate::interfaces::http::health::dto::HealthResponse;

pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}
