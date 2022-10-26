use crate::utils::{
    firebase_auth,
    load_scenario,
    mock_jwk_issuer,
    setup_mock_server,
};
use rocket_firebase_auth::{
    errors::{AuthError, InvalidJwt},
    jwt::Jwt,
};

#[tokio::test]
async fn missing_kid() {
    let token_without_kid = load_scenario("missing_kid").token;
    let firebase_auth = firebase_auth();
    let decoded_token = Jwt::verify(&token_without_kid, &firebase_auth).await;

    assert!(decoded_token.is_err());
    assert!(matches!(
        decoded_token.err().unwrap(),
        AuthError::InvalidJwt(InvalidJwt::MissingKid)
    ));
}

#[tokio::test]
// Test for when the JWK issuer return empty list
async fn missing_jwk() {
    let mock_server = setup_mock_server().await;
    let scenario = load_scenario("missing_jwk");

    // JWK issue returns empty list of jwks
    mock_jwk_issuer(Vec::new().as_slice())
        .expect(1)
        .mount(&mock_server)
        .await;

    let firebase_auth = firebase_auth();
    let decoded_token = Jwt::verify(&scenario.token, &firebase_auth).await;

    assert!(decoded_token.is_err());
    assert!(matches!(
        decoded_token.err().unwrap(),
        AuthError::InvalidJwt(InvalidJwt::MissingJwk)
    ))
}