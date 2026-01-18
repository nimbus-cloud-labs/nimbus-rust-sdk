use std::sync::Arc;

use async_trait::async_trait;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use tracing::debug;

#[cfg(feature = "env-provider")]
use crate::credentials::environment::EnvironmentCredentialProvider;

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("authorization provider did not supply a value")]
    MissingValue,
    #[error("authentication provider error: {0}")]
    Message(String),
    #[error("missing environment variable `{key}`")]
    MissingEnvVar { key: String },
    #[error("invalid value for environment variable `{key}`: {reason}")]
    InvalidEnvVar { key: String, reason: String },
    #[error("profile `{profile}` is invalid: {reason}")]
    InvalidProfile { profile: String, reason: String },
    #[error("authentication provider `{provider}` disabled (enable feature `{feature}`)")]
    ProviderDisabled {
        provider: &'static str,
        feature: &'static str,
    },
}

#[async_trait]
pub trait AuthTokenProvider: Send + Sync {
    async fn authorization_header(&self) -> Result<String, AuthError>;
}

/// Auth provider that tries multiple providers in sequence until one produces
/// an authorization header.
pub struct AuthProviderChain {
    providers: Vec<Arc<dyn AuthTokenProvider>>,
}

impl AuthProviderChain {
    pub fn new(providers: Vec<Arc<dyn AuthTokenProvider>>) -> Self {
        Self { providers }
    }

    pub fn push(&mut self, provider: Arc<dyn AuthTokenProvider>) {
        self.providers.push(provider);
    }
}

#[async_trait]
impl AuthTokenProvider for AuthProviderChain {
    async fn authorization_header(&self) -> Result<String, AuthError> {
        let mut last_error: Option<AuthError> = None;
        for (index, provider) in self.providers.iter().enumerate() {
            match provider.authorization_header().await {
                Ok(value) => {
                    debug!(
                        target: "nimbus_sdk::auth",
                        provider_index = index,
                        "authorization resolved via provider"
                    );
                    return Ok(value);
                }
                Err(error) => {
                    debug!(
                        target: "nimbus_sdk::auth",
                        provider_index = index,
                        error = %error,
                        "auth provider failed to resolve credentials"
                    );
                    last_error = Some(error);
                }
            }
        }
        Err(last_error.unwrap_or(AuthError::MissingValue))
    }
}

/// Returns the default provider chain used by the SDK. When the `env-provider`
/// feature is enabled (default), the chain contains the environment provider.
pub fn default_chain() -> Result<Arc<dyn AuthTokenProvider>, AuthError> {
    let mut providers: Vec<Arc<dyn AuthTokenProvider>> = Vec::new();

    #[cfg(feature = "env-provider")]
    {
        providers.push(Arc::new(EnvironmentCredentialProvider::new()));
    }

    if providers.is_empty() {
        Err(AuthError::ProviderDisabled {
            provider: "EnvironmentCredentialProvider",
            feature: "env-provider",
        })
    } else if providers.len() == 1 {
        Ok(providers.remove(0))
    } else {
        Ok(Arc::new(AuthProviderChain::new(providers)))
    }
}

pub struct StaticTokenProvider {
    header_value: String,
}

impl StaticTokenProvider {
    pub fn bearer(token: impl AsRef<str>) -> Self {
        Self {
            header_value: format!("Bearer {}", token.as_ref()),
        }
    }

    pub fn header(value: impl Into<String>) -> Self {
        Self {
            header_value: value.into(),
        }
    }
}

#[async_trait]
impl AuthTokenProvider for StaticTokenProvider {
    async fn authorization_header(&self) -> Result<String, AuthError> {
        if self.header_value.is_empty() {
            Err(AuthError::MissingValue)
        } else {
            Ok(self.header_value.clone())
        }
    }
}

#[derive(Debug, Clone)]
pub struct StaticKeyCredentials {
    pub access_key: String,
    pub secret_key: String,
    pub session_token: Option<String>,
}

impl StaticKeyCredentials {
    pub fn new(
        access_key: impl Into<String>,
        secret_key: impl Into<String>,
    ) -> Result<Self, AuthError> {
        let access_key = access_key.into();
        let secret_key = secret_key.into();
        validate_access_key(&access_key)?;
        validate_secret_key(&secret_key)?;
        Ok(Self {
            access_key,
            secret_key,
            session_token: None,
        })
    }

    pub fn with_session_token(mut self, token: impl Into<String>) -> Result<Self, AuthError> {
        let token = token.into();
        validate_session_token(&token)?;
        self.session_token = Some(token);
        Ok(self)
    }
}

pub struct StaticKeyProvider {
    credentials: StaticKeyCredentials,
}

impl StaticKeyProvider {
    pub fn new(credentials: StaticKeyCredentials) -> Self {
        Self { credentials }
    }
}

#[async_trait]
impl AuthTokenProvider for StaticKeyProvider {
    async fn authorization_header(&self) -> Result<String, AuthError> {
        if let Some(token) = &self.credentials.session_token {
            if token.is_empty() {
                return Err(AuthError::MissingValue);
            }
            return Ok(format!("Bearer {}", token));
        }
        if self.credentials.access_key.is_empty() || self.credentials.secret_key.is_empty() {
            return Err(AuthError::MissingValue);
        }
        let raw = format!(
            "{}:{}",
            self.credentials.access_key, self.credentials.secret_key
        );
        let encoded = STANDARD.encode(raw.as_bytes());
        Ok(format!("Basic {}", encoded))
    }
}

fn validate_access_key(value: &str) -> Result<(), AuthError> {
    if value.len() != 20 {
        return Err(AuthError::Message(
            "access key must be exactly 20 characters".to_string(),
        ));
    }
    if !value
        .chars()
        .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
    {
        return Err(AuthError::Message(
            "access key must contain only uppercase A-Z or digits 0-9".to_string(),
        ));
    }
    Ok(())
}

fn validate_secret_key(value: &str) -> Result<(), AuthError> {
    if value.len() != 44 {
        return Err(AuthError::Message(
            "secret key must be exactly 44 characters".to_string(),
        ));
    }
    if !value
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        return Err(AuthError::Message(
            "secret key must be URL-safe base64 without padding".to_string(),
        ));
    }
    Ok(())
}

fn validate_session_token(value: &str) -> Result<(), AuthError> {
    if value.len() > 512 {
        return Err(AuthError::Message(
            "session token exceeds 512 characters".to_string(),
        ));
    }
    let decoded = STANDARD
        .decode(value.as_bytes())
        .map_err(|err| AuthError::Message(format!("session token is not base64 encoded: {err}")))?;
    serde_json::from_slice::<serde_json::Value>(&decoded)
        .map_err(|err| AuthError::Message(format!("session token is not valid JSON: {err}")))?;
    Ok(())
}
