use super::support::*;
use crate::application::auth::traits::token_service::TokenService;

#[tokio::test]
async fn returns_valid_jwt_for_new_user() {
    let ctx = TestContext::new();
    ctx.given_valid_challenge();

    let result = ctx.verify().await.expect("verification succeeds");
    let user_id = ctx.jwt_service.verify(&result.token).expect("token valid");

    assert_eq!(user_id.to_string(), result.user_id);
}
