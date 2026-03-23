#[path = "../../src/security/password.rs"]
mod password;

use password::{hash_password, verify_password};

#[test]
fn hash_password_creates_a_non_empty_hash_that_verifies() {
    let raw_password = "MiPasswordSegura123!";

    let hash = hash_password(raw_password).expect("la password debería poder hashearse");

    assert!(!hash.is_empty(), "el hash no debería ser vacío");
    assert_ne!(hash, raw_password, "el hash no debería ser igual a la password original");

    verify_password(raw_password, &hash).expect("la verificación no debería fallar");
}

#[test]
fn verify_password_fails_with_an_incorrect_password() {
    let raw_password = "MiPasswordSegura123!";
    let wrong_password = "OtraPasswordDistinta456!";

    let hash = hash_password(raw_password)
        .expect("la password debería poder hashearse");

    let result = verify_password(wrong_password, &hash);

    assert!(result.is_err());
}
