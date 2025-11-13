use http::header::InvalidHeaderValue;
use thiserror::Error;
use uuid::Uuid;

use crate::auth::AuthError;
use crate::transport::TransportError;

#[derive(Debug, Error)]
pub enum SdkError {
    #[error("authentication failed: {0}")]
    Auth(#[from] AuthError),
    #[error("transport error: {0}")]
    Transport(#[from] TransportError),
    #[error("unexpected status code {status} for operation {operation}")]
    UnexpectedStatus {
        operation: &'static str,
        status: u16,
    },
    #[error("invalid path template: {0}")]
    InvalidPath(String),
    #[error("header error: {0}")]
    Header(#[from] InvalidHeaderValue),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("operation {id} timed out after {attempts} polls")]
    OperationTimeout { id: Uuid, attempts: u32 },
}
