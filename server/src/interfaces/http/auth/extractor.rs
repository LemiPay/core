use axum::{extract::FromRequestParts, http::request::Parts};
use uuid::Uuid;

use crate::domain::user::UserId;

#[allow(dead_code)]
pub struct AuthUser {
    pub user_id: UserId,
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = http::StatusCode;

    fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        let user_id = parts.extensions.get::<Uuid>().copied();

        async move {
            let user_id = user_id.ok_or(http::StatusCode::UNAUTHORIZED)?;

            Ok(AuthUser {
                user_id: UserId(user_id),
            })
        }
    }
}
