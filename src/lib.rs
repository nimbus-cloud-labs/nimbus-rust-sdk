//! Core runtime for the Nimbus Rust SDK. Generated service crates depend on this
//! library for HTTP transport, authentication middleware, pagination helpers,
//! and long-running operation waiters.

pub mod auth;
pub mod client;
pub mod error;
pub mod idempotency;
pub mod lro;
pub mod paginator;
pub mod transport;

pub use auth::{AuthError, AuthTokenProvider, StaticTokenProvider};
pub use client::{
    BuildError, NimbusClient, OperationResult, OperationSpec, PaginationSpec, SdkConfig,
    SdkConfigBuilder,
};
pub use error::SdkError;
pub use idempotency::{default_provider as default_idempotency_provider, IdempotencyTokenProvider};
pub use lro::{LroWaiter, OperationHandle, OperationStatus, OperationStatusClient};
pub use paginator::Paginator;
pub use transport::{ReqwestTransport, SdkHttpMethod, Transport};
