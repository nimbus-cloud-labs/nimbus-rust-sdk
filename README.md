# Nimbus Rust SDK

Nimbus Rust SDK provides strongly typed clients generated from shared Smithy models. This package is the canonical reference for SDK semantics (auth, pagination, LRO, error mapping).

## Install
Add the core runtime and service crates to your `Cargo.toml`:

```toml
[dependencies]
nimbus-sdk-core = { version = "0.1.0" }
nimbus-sdk-iam = { version = "0.1.0" }
```

## Quickstart

```rust
use std::sync::Arc;

use nimbus_sdk_core::client::SdkConfigBuilder;
use nimbus_sdk_core::auth::StaticTokenProvider;
use nimbus_sdk_iam::IamClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = SdkConfigBuilder::default()
        .endpoint("https://api.nimbus.eu")
        .auth(Arc::new(StaticTokenProvider::bearer("token")))
        .build()?;

    let client = IamClient::new(config.into());
    let response = client.emit_token(&serde_json::json!({
        "urn": "urn:nimbus:iam::123",
        "typ": "access"
    }))
    .await?;

    println!("{}", response["token"]);
    Ok(())
}
```

## Authentication
- **Environment provider (default):** Reads `NIMBUS_*` variables from the environment.
- **Static access keys:** Provide access/secret keys directly when you do not want environment resolution.

```rust
use std::sync::Arc;

use nimbus_sdk_core::auth::{StaticKeyCredentials, StaticKeyProvider};
use nimbus_sdk_core::client::SdkConfigBuilder;

let credentials = StaticKeyCredentials::new(
    "ABCDEFGHIJKLMNOPQRST",
    "abcdEFGHijklMNOPqrstUVWXyz0123456789ABCDabcd",
)?;
let config = SdkConfigBuilder::default()
    .endpoint("https://api.nimbus.eu")
    .auth(Arc::new(StaticKeyProvider::new(credentials)))
    .build()?;
```

## Pagination and LRO
Generated clients expose paginator helpers and `.wait()` for long-running operations. These follow Smithy traits and align across languages.

## Development
- Regenerate SDK clients from Smithy models using the bundled generator.
- Run `cargo fmt --all` and `cargo clippy --all-targets --all-features -- -D warnings` before submitting changes.
