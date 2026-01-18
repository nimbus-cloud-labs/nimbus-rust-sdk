# Nimbus Rust SDK

The Rust SDK provides strongly typed clients generated from the shared Smithy
models. Packages under this directory are published together to ensure feature
parity with the TypeScript distribution.

## Credential environment variables
The Rust crates rely on `NimbusCredentialProvider` to resolve authentication
material. By default the provider reads the environment variables documented in
[`docs/sdk/glossary.md`](../../docs/sdk/glossary.md#credential-environment-variables).
Profiles are resolved by appending the uppercase suffix described in the
glossary (for example, `NIMBUS_ACCESS_KEY_SRE`).

Unlike Cargo configuration flags, environment variables are read at runtime by
the SDK. Flags such as `cargo run --features` or `cargo run -- --access-key`
only affect the build invocation or a specific binary execution and do not
propagate to dependent crates. Environment variables keep credentials outside
shell history and work for `cargo test`, integration tools, and compiled
artifacts executed later.

When troubleshooting credential resolution, refer to the
[glossary troubleshooting notes](../../docs/sdk/glossary.md#troubleshooting).
The [credential provider precedence overview](../../docs/sdk/credential-provider-strategy.md#credential-provider-precedence)
contains a diagram and comparison tables summarizing provider overrides, caching, and error reporting.

## Static access keys
Applications can pass access key credentials directly when they do not want to
load environment variables:

```rust
use std::sync::Arc;

use nimbus_sdk_core::auth::{StaticKeyCredentials, StaticKeyProvider};
use nimbus_sdk_core::client::SdkConfigBuilder;

let credentials = StaticKeyCredentials::new(
    "ABCDEFGHIJKLMNOPQRST",
    "abcdEFGHijklMNOPqrstUVWXyz0123456789ABCDabcd",
)?
.with_session_token("eyJ0b2tlbiI6ICJ0ZXN0In0=")?;
let config = SdkConfigBuilder::default()
    .auth(Arc::new(StaticKeyProvider::new(credentials)))
    .endpoint("https://api.nimbus.eu")
    .build()?;
```

## Instance metadata credentials

Workloads running on Nimbus compute instances can retrieve rotating credentials
from the metadata service described in
[`docs/compute/metadata-service.md`](../../docs/compute/metadata-service.md).
The document covers the required headers, token time-to-live limits, network
allowlisting, and the 30 requests per second rate policy enforced by the
platform. Integrations should reuse the shared HTTP client stack so retries and
token refresh logic remain consistent across SDK components.
