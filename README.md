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
