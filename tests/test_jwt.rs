extern crate core;

mod common;

use crate::common::utils::{
    load_scenario,
    mock_jwk_issuer,
    setup_mock_server,
    JWKS_URL,
};
use rocket_firebase_auth::{
    errors::{AuthError, InvalidJwt},
    firebase_auth::FirebaseAuth,
    jwk::Jwk,
    jwt::Jwt,
};

#[tokio::test]
async fn missing_kid() {
    let token_without_kid = load_scenario("missing_kid").token;
    let firebase_auth = FirebaseAuth::default();
    let decoded_token = Jwt::verify(&token_without_kid, &firebase_auth).await;

    assert!(decoded_token.is_err());
    assert!(matches!(
        decoded_token.err().unwrap(),
        AuthError::InvalidJwt(InvalidJwt::MissingKid)
    ));
}

// Test for when the JWK issuer return empty list
#[tokio::test]
async fn missing_jwk() {
    let mock_server = setup_mock_server().await;
    let scenario = load_scenario("missing_jwk");

    // JWK issue returns empty list of jwks
    mock_jwk_issuer(Vec::new().as_slice())
        .expect(1)
        .mount(&mock_server)
        .await;

    let firebase_auth = FirebaseAuth::default();
    let decoded_token =
        Jwt::verify_with_jwks_url(&scenario.token, JWKS_URL, &firebase_auth)
            .await;

    assert!(decoded_token.is_err());
    assert!(matches!(
        decoded_token.err().unwrap(),
        AuthError::InvalidJwt(InvalidJwt::MatchingJwkNotFound)
    ))
}

#[tokio::test]
async fn success() {
    let mock_server = setup_mock_server().await;
    let scenario = load_scenario("success");

    let jwk = Jwk {
        e:   "AQAB".to_string(),
        alg: "RS256".to_string(),
        kty: "RSA".to_string(),
        kid: scenario.kid,
        n:   scenario.jwk_n,
    };

    mock_jwk_issuer(vec![jwk].as_slice())
        .expect(1)
        .mount(&mock_server)
        .await;

    let firebase_auth =
        FirebaseAuth::try_from_filename("./tests/.env.test", "FIREBASE_CREDS")
            .unwrap();
    let decoded_token =
        Jwt::verify_with_jwks_url(&scenario.token, JWKS_URL, &firebase_auth)
            .await;

    assert!(decoded_token.is_ok());

    let decoded_token = decoded_token.unwrap();

    assert_eq!(decoded_token.uid, "some-uid");
    assert!(decoded_token.expires_at > decoded_token.issued_at);
}
