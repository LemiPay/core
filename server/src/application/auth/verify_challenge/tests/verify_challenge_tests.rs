use super::support::*;
use crate::application::auth::verify_challenge::dto::VerificationInput;
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
async fn rejects_when_nonce_mismatch_without_client_issued_at() {
    let ctx = TestContext::new();
    ctx.given_challenge_with_nonce("other-nonce");

    // Sin issued_at del cliente no hay fallback: el cache tiene otro nonce.
    let result = ctx
        .verify_with(VerificationInput {
            email: Some(EMAIL.to_string()),
            name: Some(NAME.to_string()),
            allow_linking: false,
            address: ADDRESS.to_string(),
            nonce: NONCE.to_string(),
            signature: SIGNATURE.to_string(),
            issued_at: None,
        })
        .await;

    assert!(matches!(
        result,
        Err(AppError::Forbidden(msg))
            if msg == "Nonce inválido" || msg == "Sesión expirada o desafío no solicitado"
    ));
}

#[tokio::test]
async fn accepts_client_issued_at_when_cache_missing() {
    let ctx = TestContext::new();
    // Sin cache: el verify debe funcionar con issued_at fresco del cliente.
    let issued_at = chrono::Utc::now().to_rfc3339();
    ctx.web3.set_signature_valid(true);

    let result = ctx
        .verify_with(VerificationInput {
            email: Some(EMAIL.to_string()),
            name: Some(NAME.to_string()),
            allow_linking: false,
            address: ADDRESS.to_string(),
            nonce: NONCE.to_string(),
            signature: SIGNATURE.to_string(),
            issued_at: Some(issued_at),
        })
        .await
        .expect("verification succeeds with client issued_at");

    assert_eq!(result.user_id, ctx.new_user_id.to_string());
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
async fn rejects_when_email_already_registered_without_wallet() {
    let ctx = TestContext::new();
    let user = ctx.given_existing_user();
    ctx.given_valid_challenge();

    let result = ctx.verify().await;

    assert!(matches!(
        result,
        Err(AppError::BadRequest(msg)) if msg.contains("email ya tiene una cuenta")
            || msg.contains("Email ya está asociado")
    ));
    assert!(ctx.wallet_repo.wallet_for_user(user.id).is_none());
}

#[tokio::test]
async fn logs_in_when_email_registered_and_wallet_linked() {
    let ctx = TestContext::new();
    let user = ctx.given_existing_user();
    ctx.given_wallet_for_user(&user, ADDRESS);
    ctx.given_valid_challenge();

    let result = ctx.verify().await.expect("verification succeeds");

    assert_eq!(result.user_id, user.id.to_string());
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
    assert_eq!(saved_user.name, NAME);

    let wallet = ctx
        .wallet_repo
        .wallet_for_user(ctx.new_user_id)
        .expect("wallet created");
    assert_eq!(wallet.address, ADDRESS);
    assert_eq!(wallet.user_id, ctx.new_user_id);

    assert_eq!(result.user_id, ctx.new_user_id.to_string());
}

#[tokio::test]
async fn logs_in_with_wallet_address_when_email_missing() {
    let ctx = TestContext::new();
    let user = ctx.given_existing_user();
    ctx.given_wallet_for_user(&user, ADDRESS);
    ctx.given_valid_challenge();

    let result = ctx
        .verify_with(VerificationInput {
            email: None,
            name: None,
            allow_linking: false,
            address: ADDRESS.to_string(),
            nonce: NONCE.to_string(),
            signature: SIGNATURE.to_string(),
            issued_at: Some(ISSUED_AT.to_string()),
        })
        .await
        .expect("verification succeeds");

    assert_eq!(result.user_id, user.id.to_string());
    assert!(ctx.auth_repo.saved_users().is_empty());
}

#[tokio::test]
async fn links_wallet_to_existing_user_when_allow_linking() {
    let ctx = TestContext::new();
    let user = ctx.given_existing_user();
    ctx.given_valid_challenge();

    let result = ctx
        .verify_with(VerificationInput {
            email: Some(EMAIL.to_string()),
            name: Some(NAME.to_string()),
            allow_linking: true,
            address: ADDRESS.to_string(),
            nonce: NONCE.to_string(),
            signature: SIGNATURE.to_string(),
            issued_at: Some(ISSUED_AT.to_string()),
        })
        .await
        .expect("verification succeeds");

    assert_eq!(result.user_id, user.id.to_string());
    let wallet = ctx
        .wallet_repo
        .wallet_for_user(user.id)
        .expect("wallet created");
    assert_eq!(wallet.address, ADDRESS);
    assert!(ctx.auth_repo.saved_users().is_empty());
}

#[tokio::test]
async fn clears_challenge_after_success() {
    let ctx = TestContext::new();
    ctx.given_valid_challenge();

    ctx.verify().await.expect("verification succeeds");

    assert!(!ctx.web3.has_challenge(ADDRESS));
}
