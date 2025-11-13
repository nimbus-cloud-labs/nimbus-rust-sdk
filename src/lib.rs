//! Core runtime for the Nimbus Rust SDK. Generated service crates depend on this
//! library for HTTP transport, authentication middleware, pagination helpers,
//! and long-running operation waiters.

pub mod auth;
pub mod client;
#[cfg(feature = "env-provider")]
pub mod credentials;
pub mod error;
pub mod idempotency;
pub mod lro;
pub mod paginator;
pub mod transport;

pub use auth::{
    default_chain as default_auth_chain, AuthError, AuthProviderChain, AuthTokenProvider,
    StaticTokenProvider,
};
pub use client::{
    AdditionalSuccessResponseSpec, BuildError, NimbusClient, OperationResult, OperationSpec,
    PaginationSpec, SdkConfig, SdkConfigBuilder,
};
pub use error::SdkError;
pub use idempotency::{default_provider as default_idempotency_provider, IdempotencyTokenProvider};
pub use lro::{LroWaiter, OperationHandle, OperationStatus, OperationStatusClient};
pub use paginator::Paginator;
pub use transport::{ReqwestTransport, SdkHttpMethod, Transport};
