use hmac::{Hmac, Mac};
use http::HeaderMap;
use nimbus_sdk_core::auth::{AuthTokenProvider, StaticKeyCredentials, StaticKeyProvider};
use sha2::Sha256;

const ACCESS_KEY: &str = "ABCDEFGHIJKLMNOPQRSTUVWX12";
const SECRET_KEY: &str = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
const PAYLOAD: &[u8] = b"payload";

#[tokio::test]
async fn uses_hmac_when_no_session_token() {
    let credentials = StaticKeyCredentials::new(ACCESS_KEY, SECRET_KEY).expect("valid credentials");
    let provider = StaticKeyProvider::new(credentials);

    let mut headers = HeaderMap::new();
    provider
        .apply(&mut headers, PAYLOAD)
        .await
        .expect("authorization headers");
    assert_hmac_headers(&headers, ACCESS_KEY, SECRET_KEY, PAYLOAD);
}

#[tokio::test]
async fn uses_bearer_auth_when_session_token_is_set() {
    let credentials = StaticKeyCredentials::new(ACCESS_KEY, SECRET_KEY)
        .expect("valid credentials")
        .with_session_token("eyJ0b2tlbiI6ICJ0ZXN0In0=")
        .expect("valid session token");
    let provider = StaticKeyProvider::new(credentials);

    let mut headers = HeaderMap::new();
    provider
        .apply(&mut headers, PAYLOAD)
        .await
        .expect("authorization header");
    let header = headers
        .get(http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .expect("missing auth header");
    assert_eq!(header, "Bearer eyJ0b2tlbiI6ICJ0ZXN0In0=");
    assert!(headers.get("x-nimbus-signature").is_none());
    assert!(headers.get("x-nimbus-date").is_none());
}

type HmacSha256 = Hmac<Sha256>;

fn assert_hmac_headers(headers: &HeaderMap, access_key: &str, secret_key: &str, payload: &[u8]) {
    let signature = headers
        .get("x-nimbus-signature")
        .and_then(|value| value.to_str().ok())
        .expect("missing signature header");
    let date = headers
        .get("x-nimbus-date")
        .and_then(|value| value.to_str().ok())
        .expect("missing date header");
    let expected = compute_signature(access_key, secret_key, payload, date);
    assert_eq!(signature, expected);
    assert!(headers.get(http::header::AUTHORIZATION).is_none());
}

fn compute_signature(access_key: &str, secret_key: &str, payload: &[u8], date: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(secret_key.as_bytes()).expect("hmac init");
    mac.update(payload);
    mac.update(date.as_bytes());
    format!(
        "{}:{}",
        access_key,
        hex::encode(mac.finalize().into_bytes())
    )
}
