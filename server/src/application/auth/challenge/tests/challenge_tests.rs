use super::support::*;
use crate::interfaces::http::error::AppError;
use alloy::primitives::Address;

#[test]
fn generates_challenge_and_caches_it() {
    let ctx = TestContext::new();

    let result = ctx.generate(ADDRESS).expect("challenge generated");

    assert_eq!(result.nonce, NONCE);
    let parsed: Address = ADDRESS.parse().expect("valid address");
    assert_eq!(
        result.message,
        ctx.web3
            .expected_message(&parsed, &NONCE.to_string(), &ISSUED_AT.to_string())
    );

    let cached = ctx.web3.challenge_for(ADDRESS).expect("challenge cached");
    assert_eq!(cached.nonce, NONCE);
    assert_eq!(cached.issued_at, ISSUED_AT);
}

#[test]
fn rejects_invalid_address() {
    let ctx = TestContext::new();

    let result = ctx.generate("not-an-address");

    assert!(matches!(
        result,
        Err(AppError::BadRequest(msg)) if msg == "Dirección Ethereum inválida"
    ));
    assert!(ctx.web3.challenge_for("not-an-address").is_none());
}
