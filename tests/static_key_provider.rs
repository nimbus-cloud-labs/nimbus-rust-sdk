use nimbus_sdk_core::auth::{AuthTokenProvider, StaticKeyCredentials, StaticKeyProvider};

#[tokio::test]
async fn uses_basic_auth_when_no_session_token() {
    let credentials = StaticKeyCredentials::new(
        "ABCDEFGHIJKLMNOPQRST",
        "abcdEFGHijklMNOPqrstUVWXyz0123456789ABCDabcd",
    )
    .expect("valid credentials");
    let provider = StaticKeyProvider::new(credentials);

    let header = provider
        .authorization_header()
        .await
        .expect("authorization header");

    assert!(header.starts_with("Basic "));
}

#[tokio::test]
async fn uses_bearer_auth_when_session_token_is_set() {
    let credentials = StaticKeyCredentials::new(
        "ABCDEFGHIJKLMNOPQRST",
        "abcdEFGHijklMNOPqrstUVWXyz0123456789ABCDabcd",
    )
    .expect("valid credentials")
    .with_session_token("eyJ0b2tlbiI6ICJ0ZXN0In0=")
    .expect("valid session token");
    let provider = StaticKeyProvider::new(credentials);

    let header = provider
        .authorization_header()
        .await
        .expect("authorization header");

    assert_eq!(header, "Bearer eyJ0b2tlbiI6ICJ0ZXN0In0=");
}
