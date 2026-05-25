use super::support::*;
use crate::interfaces::http::error::AppError;

#[tokio::test]
async fn rejects_when_challenge_missing() {
    let ctx = TestContext::new();

    let result = ctx.verify().await;

    assert!(matches!(
        result,
        Err(AppError::Forbidden(msg)) if msg == "Sesión expirada o desafío no solicitado"
    ));
}

#[tokio::test]
async fn rejects_when_nonce_mismatch() {
    let ctx = TestContext::new();
    ctx.given_challenge_with_nonce("other-nonce");

    let result = ctx.verify().await;

    assert!(matches!(
        result,
        Err(AppError::Forbidden(msg)) if msg == "Nonce inválido"
    ));
}

#[tokio::test]
async fn rejects_invalid_signature() {
    let ctx = TestContext::new();
    ctx.given_valid_challenge();
    ctx.web3.set_signature_valid(false);

    let result = ctx.verify().await;

    assert!(matches!(
        result,
        Err(AppError::Forbidden(msg)) if msg == "Firma criptográfica inválida"
    ));
}

#[tokio::test]
async fn creates_wallet_for_existing_user_without_wallet() {
    let ctx = TestContext::new();
    let user = ctx.given_existing_user();
    ctx.given_valid_challenge();

    let result = ctx.verify().await.expect("verification succeeds");

    assert_eq!(result.user_id, user.id.to_string());
    let wallet = ctx
        .wallet_repo
        .wallet_for_user(user.id)
        .expect("wallet created");
    assert_eq!(wallet.address, ADDRESS);
}

#[tokio::test]
async fn does_not_duplicate_wallet_for_existing_user_with_wallet() {
    let ctx = TestContext::new();
    let user = ctx.given_existing_user();
    ctx.given_wallet_for_user(&user, ADDRESS);
    ctx.given_valid_challenge();

    ctx.verify().await.expect("verification succeeds");

    let wallets = ctx.wallet_repo.wallets_for_user(user.id);
    assert_eq!(wallets.len(), 1);
}

#[tokio::test]
async fn creates_user_and_wallet_for_new_user() {
    let ctx = TestContext::new();
    ctx.given_valid_challenge();

    let result = ctx.verify().await.expect("verification succeeds");

    let saved_user = ctx
        .auth_repo
        .saved_users()
        .into_iter()
        .next()
        .expect("user saved");
    assert_eq!(saved_user.email, EMAIL);
    assert_eq!(saved_user.name, ADDRESS);

    let wallet = ctx
        .wallet_repo
        .wallet_for_user(ctx.new_user_id)
        .expect("wallet created");
    assert_eq!(wallet.address, ADDRESS);
    assert_eq!(wallet.user_id, ctx.new_user_id);

    assert_eq!(result.user_id, ctx.new_user_id.to_string());
}

#[tokio::test]
async fn clears_challenge_after_success() {
    let ctx = TestContext::new();
    ctx.given_valid_challenge();

    ctx.verify().await.expect("verification succeeds");

    assert!(!ctx.web3.has_challenge(ADDRESS));
}
