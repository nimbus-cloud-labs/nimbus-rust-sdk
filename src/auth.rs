use std::sync::Arc;

use async_trait::async_trait;
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
