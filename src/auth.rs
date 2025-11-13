use async_trait::async_trait;

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("authorization provider did not supply a value")]
    MissingValue,
    #[error("authentication provider error: {0}")]
    Message(String),
}

#[async_trait]
pub trait AuthTokenProvider: Send + Sync {
    async fn authorization_header(&self) -> Result<String, AuthError>;
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
