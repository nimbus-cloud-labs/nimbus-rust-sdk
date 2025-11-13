#![cfg_attr(not(feature = "env-provider"), allow(unused_imports))]

use base64::engine::general_purpose::STANDARD;
use base64::Engine;

#[cfg(feature = "env-provider")]
mod enabled {
    use super::*;
    use nimbus_sdk_core::auth::{AuthError, AuthTokenProvider};
    use nimbus_sdk_core::credentials::environment::EnvironmentCredentialProvider;

    const ACCESS_KEY: &str = "ABCDEFGHIJKLMNOPQRST";
    const SECRET_KEY: &str = "abcdEFGHijklMNOPqrstUVWXyz0123456789ABCDabcd";
    const REGION: &str = "eu-north-2";

    #[tokio::test]
    async fn uses_profile_specific_variables() {
        let session = STANDARD.encode(r#"{"token":"demo"}"#);
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

        let header = provider.authorization_header().await.unwrap();
        assert_eq!(header, format!("Bearer {}", session));
    }

    #[tokio::test]
    async fn falls_back_to_basic_auth_without_session_token() {
        let provider = EnvironmentCredentialProvider::from_snapshot(vec![
            ("NIMBUS_ACCESS_KEY".to_string(), ACCESS_KEY.to_string()),
            ("NIMBUS_SECRET_KEY".to_string(), SECRET_KEY.to_string()),
            ("NIMBUS_REGION".to_string(), REGION.to_string()),
        ]);

        let header = provider.authorization_header().await.unwrap();
        let expected = format!(
            "Basic {}",
            STANDARD.encode(format!("{ACCESS_KEY}:{SECRET_KEY}"))
        );
        assert_eq!(header, expected);
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

        let err = provider.authorization_header().await.unwrap_err();
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

        let err = provider.authorization_header().await.unwrap_err();
        match err {
            AuthError::InvalidEnvVar { key, .. } => {
                assert_eq!(key, "NIMBUS_SESSION_TOKEN");
            }
            other => panic!("expected InvalidEnvVar, received {other:?}"),
        }
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
