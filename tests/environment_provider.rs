#![cfg_attr(not(feature = "env-provider"), allow(unused_imports))]

use base64::Engine;
use hmac::{Hmac, Mac};
use http::HeaderMap;
use sha2::Sha256;

#[cfg(feature = "env-provider")]
mod enabled {
    use super::*;
    use nimbus_sdk_core::auth::{AuthError, AuthTokenProvider};
    use nimbus_sdk_core::credentials::environment::EnvironmentCredentialProvider;

    const ACCESS_KEY: &str = "ABCDEFGHIJKLMNOPQRSTUVWX12";
    const SECRET_KEY: &str = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
    const REGION: &str = "eu-north-2";
    const PAYLOAD: &[u8] = b"payload";

    #[tokio::test]
    async fn uses_profile_specific_variables() {
        let session = base64::engine::general_purpose::STANDARD.encode(r#"{"token":"demo"}"#);
        let provider = EnvironmentCredentialProvider::from_snapshot(vec![
            ("NIMBUS_PROFILE".to_string(), "analytics".to_string()),
            (
                "NIMBUS_PROFILE_ANALYTICS_ACCESS_KEY".to_string(),
                ACCESS_KEY.to_string(),
            ),
            (
                "NIMBUS_PROFILE_ANALYTICS_SECRET_KEY".to_string(),
                SECRET_KEY.to_string(),
            ),
            (
                "NIMBUS_PROFILE_ANALYTICS_REGION".to_string(),
                REGION.to_string(),
            ),
            (
                "NIMBUS_PROFILE_ANALYTICS_SESSION_TOKEN".to_string(),
                session.clone(),
            ),
            (
                "NIMBUS_ACCESS_KEY".to_string(),
                "ZZZZZZZZZZZZZZZZZZZZ".to_string(),
            ),
            ("NIMBUS_SECRET_KEY".to_string(), SECRET_KEY.to_string()),
            ("NIMBUS_REGION".to_string(), REGION.to_string()),
        ]);

        let mut headers = HeaderMap::new();
        provider.apply(&mut headers, PAYLOAD).await.unwrap();
        let header = headers
            .get(http::header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .unwrap();
        assert_eq!(header, format!("Bearer {}", session));
        assert!(headers.get("x-nimbus-signature").is_none());
        assert!(headers.get("x-nimbus-date").is_none());
    }

    #[tokio::test]
    async fn falls_back_to_global_variables_when_profile_missing() {
        let provider = EnvironmentCredentialProvider::from_snapshot(vec![
            ("NIMBUS_PROFILE".to_string(), "analytics".to_string()),
            ("NIMBUS_ACCESS_KEY".to_string(), ACCESS_KEY.to_string()),
            ("NIMBUS_SECRET_KEY".to_string(), SECRET_KEY.to_string()),
            ("NIMBUS_REGION".to_string(), REGION.to_string()),
        ]);

        let mut headers = HeaderMap::new();
        provider.apply(&mut headers, PAYLOAD).await.unwrap();
        assert_hmac_headers(&headers, ACCESS_KEY, SECRET_KEY, PAYLOAD);
    }

    #[tokio::test]
    async fn falls_back_to_hmac_without_session_token() {
        let provider = EnvironmentCredentialProvider::from_snapshot(vec![
            ("NIMBUS_ACCESS_KEY".to_string(), ACCESS_KEY.to_string()),
            ("NIMBUS_SECRET_KEY".to_string(), SECRET_KEY.to_string()),
            ("NIMBUS_REGION".to_string(), REGION.to_string()),
        ]);

        let mut headers = HeaderMap::new();
        provider.apply(&mut headers, PAYLOAD).await.unwrap();
        assert_hmac_headers(&headers, ACCESS_KEY, SECRET_KEY, PAYLOAD);
    }

    #[tokio::test]
    async fn errors_on_missing_required_variable() {
        let provider = EnvironmentCredentialProvider::from_snapshot(vec![
            ("NIMBUS_PROFILE".to_string(), "sre".to_string()),
            (
                "NIMBUS_PROFILE_SRE_ACCESS_KEY".to_string(),
                ACCESS_KEY.to_string(),
            ),
            ("NIMBUS_PROFILE_SRE_REGION".to_string(), REGION.to_string()),
        ]);

        let mut headers = HeaderMap::new();
        let err = provider.apply(&mut headers, PAYLOAD).await.unwrap_err();
        match err {
            AuthError::MissingEnvVar { key } => {
                assert_eq!(key, "NIMBUS_PROFILE_SRE_SECRET_KEY");
            }
            other => panic!("expected MissingEnvVar, received {other:?}"),
        }
    }

    #[tokio::test]
    async fn errors_on_malformed_session_token() {
        let provider = EnvironmentCredentialProvider::from_snapshot(vec![
            ("NIMBUS_ACCESS_KEY".to_string(), ACCESS_KEY.to_string()),
            ("NIMBUS_SECRET_KEY".to_string(), SECRET_KEY.to_string()),
            ("NIMBUS_REGION".to_string(), REGION.to_string()),
            (
                "NIMBUS_SESSION_TOKEN".to_string(),
                "!!!not-base64!!!".to_string(),
            ),
        ]);

        let mut headers = HeaderMap::new();
        let err = provider.apply(&mut headers, PAYLOAD).await.unwrap_err();
        match err {
            AuthError::InvalidEnvVar { key, .. } => {
                assert_eq!(key, "NIMBUS_SESSION_TOKEN");
            }
            other => panic!("expected InvalidEnvVar, received {other:?}"),
        }
    }

    type HmacSha256 = Hmac<Sha256>;

    fn assert_hmac_headers(
        headers: &HeaderMap,
        access_key: &str,
        secret_key: &str,
        payload: &[u8],
    ) {
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
}

#[cfg(not(feature = "env-provider"))]
mod disabled {
    use super::*;
    use nimbus_sdk_core::auth::{self, AuthError};

    #[test]
    fn default_chain_requires_env_provider_feature() {
        let err = auth::default_chain().expect_err("feature should be disabled");
        match err {
            AuthError::ProviderDisabled { feature, .. } => {
                assert_eq!(feature, "env-provider");
            }
            other => panic!("expected ProviderDisabled error, got {other:?}"),
        }
    }
}
