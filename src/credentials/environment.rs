use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use async_trait::async_trait;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use tracing::{debug, warn};

use crate::auth::{AuthError, AuthTokenProvider};
use http::HeaderMap;

const PROFILE_ENV: &str = "NIMBUS_PROFILE";
const PROFILE_PREFIX: &str = "NIMBUS_PROFILE";
const GLOBAL_PREFIX: &str = "NIMBUS";
const ACCESS_KEY: &str = "ACCESS_KEY";
const SECRET_KEY: &str = "SECRET_KEY";
const REGION_KEY: &str = "REGION";
const SESSION_TOKEN_KEY: &str = "SESSION_TOKEN";

/// Loads Nimbus credentials from process environment variables following the
/// `NIMBUS_PROFILE_<NAME>_*` naming convention with a global fallback.
///
/// Profiles are selected through the `NIMBUS_PROFILE` environment variable. When
/// unset, the provider falls back to the default profile using the unsuffixed
/// `NIMBUS_<KEY>` variables documented in `docs/sdk/glossary.md`.
#[derive(Clone, Default)]
pub struct EnvironmentCredentialProvider {
    source: EnvSource,
}

#[derive(Clone, Default)]
enum EnvSource {
    #[default]
    Real,
    Snapshot(Arc<HashMap<String, String>>),
}

impl EnvSource {
    fn get(&self, key: &str) -> Option<String> {
        match self {
            EnvSource::Real => std::env::var(key).ok(),
            EnvSource::Snapshot(values) => values.get(key).cloned(),
        }
    }
}

impl EnvironmentCredentialProvider {
    /// Creates a provider that reads directly from `std::env`.
    pub fn new() -> Self {
        Self {
            source: EnvSource::Real,
        }
    }

    /// Creates a provider backed by the supplied key/value snapshot. Primarily
    /// used by tests to avoid mutating the real environment.
    pub fn from_snapshot<K, V, I>(values: I) -> Self
    where
        K: Into<String>,
        V: Into<String>,
        I: IntoIterator<Item = (K, V)>,
    {
        let map = values
            .into_iter()
            .map(|(key, value)| (key.into(), value.into()))
            .collect::<HashMap<_, _>>();
        Self {
            source: EnvSource::Snapshot(Arc::new(map)),
        }
    }

    fn resolve(&self) -> Result<Credentials, AuthError> {
        let profile = self
            .source
            .get(PROFILE_ENV)
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());

        if let Some(profile_name) = profile {
            let normalized = normalize_profile(&profile_name)?;
            debug!(
                target: "nimbus_sdk::auth",
                profile = %normalized,
                "resolving credentials from profile-scoped environment variables"
            );
            self.load_scope(Scope::Profile(&normalized))
        } else {
            debug!(
                target: "nimbus_sdk::auth",
                "resolving credentials from global environment variables"
            );
            self.load_scope(Scope::Global)
        }
    }

    fn load_scope(&self, scope: Scope<'_>) -> Result<Credentials, AuthError> {
        let access_key = self.read_required(scope, ACCESS_KEY, validate_access_key)?;
        let secret_key = self.read_required(scope, SECRET_KEY, validate_secret_key)?;
        let region = self.read_required(scope, REGION_KEY, validate_region)?;
        let session_token = self.read_optional_session_token(scope)?;

        debug!(
            target: "nimbus_sdk::auth",
            scope = %scope,
            access_key = %Redacted(&access_key),
            region = %region,
            has_session_token = session_token.is_some(),
            "successfully loaded credentials from environment"
        );

        Ok(Credentials {
            access_key,
            secret_key,
            region,
            session_token,
        })
    }

    fn read_required<F>(
        &self,
        scope: Scope<'_>,
        suffix: &str,
        validate: F,
    ) -> Result<String, AuthError>
    where
        F: Fn(&str) -> Result<(), &'static str>,
    {
        let candidates = scope.candidate_keys(suffix);
        for (index, key) in candidates.iter().enumerate() {
            match self.source.get(key) {
                Some(value) => {
                    let trimmed = value.trim().to_string();
                    if trimmed.is_empty() {
                        warn!(
                            target: "nimbus_sdk::auth",
                            key = %key,
                            "environment variable is set but empty"
                        );
                        return Err(AuthError::InvalidEnvVar {
                            key: key.to_string(),
                            reason: "value must not be empty".to_string(),
                        });
                    }
                    validate(&trimmed).map_err(|reason| {
                        warn!(
                            target: "nimbus_sdk::auth",
                            key = %key,
                            reason = %reason,
                            "environment variable failed validation"
                        );
                        AuthError::InvalidEnvVar {
                            key: key.to_string(),
                            reason: reason.to_string(),
                        }
                    })?;
                    if index > 0 {
                        debug!(
                            target: "nimbus_sdk::auth",
                            key = %key,
                            "falling back to global credential"
                        );
                    }
                    return Ok(trimmed);
                }
                None => continue,
            }
        }

        warn!(
            target: "nimbus_sdk::auth",
            keys = %candidates.join(", "),
            "required environment variable missing"
        );
        Err(AuthError::MissingEnvVar {
            key: candidates
                .first()
                .cloned()
                .unwrap_or_else(|| format!("{GLOBAL_PREFIX}_{suffix}")),
        })
    }

    fn read_optional_session_token(&self, scope: Scope<'_>) -> Result<Option<String>, AuthError> {
        let candidates = scope.candidate_keys(SESSION_TOKEN_KEY);
        for (index, key) in candidates.iter().enumerate() {
            if let Some(value) = self.source.get(key) {
                let trimmed = value.trim().to_string();
                if trimmed.is_empty() {
                    continue;
                }
                self.validate_session_token(key, &trimmed)?;
                if index > 0 {
                    debug!(
                        target: "nimbus_sdk::auth",
                        key = %key,
                        "falling back to global session token"
                    );
                }
                return Ok(Some(trimmed));
            }
        }
        Ok(None)
    }

    fn validate_session_token(&self, key: &str, value: &str) -> Result<(), AuthError> {
        if value.len() > 512 {
            return Err(AuthError::InvalidEnvVar {
                key: key.to_string(),
                reason: "value exceeds 512 characters".to_string(),
            });
        }
        let decoded = STANDARD
            .decode(value)
            .map_err(|err| AuthError::InvalidEnvVar {
                key: key.to_string(),
                reason: format!("value is not base64 encoded: {err}"),
            })?;
        let json = String::from_utf8(decoded).map_err(|err| AuthError::InvalidEnvVar {
            key: key.to_string(),
            reason: format!("value is not valid UTF-8: {err}"),
        })?;
        if json.trim_start().is_empty() {
            return Err(AuthError::InvalidEnvVar {
                key: key.to_string(),
                reason: "decoded payload is empty".to_string(),
            });
        }
        let first = json.trim_start().chars().next().unwrap_or(' ');
        if first != '{' && first != '[' {
            return Err(AuthError::InvalidEnvVar {
                key: key.to_string(),
                reason: "decoded payload is not JSON".to_string(),
            });
        }
        Ok(())
    }
}

#[async_trait]
impl AuthTokenProvider for EnvironmentCredentialProvider {
    async fn apply(&self, headers: &mut HeaderMap, payload: &[u8]) -> Result<(), AuthError> {
        let credentials = self.resolve()?;
        credentials.apply(headers, payload)
    }
}

#[derive(Debug)]
struct Credentials {
    access_key: String,
    secret_key: String,
    #[allow(dead_code)]
    region: String,
    session_token: Option<String>,
}

impl Credentials {
    fn apply(&self, headers: &mut HeaderMap, payload: &[u8]) -> Result<(), AuthError> {
        if let Some(token) = &self.session_token {
            debug!(
                target: "nimbus_sdk::auth",
                scheme = "Bearer",
                "using session token for authorization header"
            );
            let header = http::header::HeaderValue::from_str(&format!("Bearer {}", token))
                .map_err(|error| {
                    AuthError::Message(format!("invalid authorization header: {error}"))
                })?;
            headers.insert(http::header::AUTHORIZATION, header);
            return Ok(());
        }

        debug!(
            target: "nimbus_sdk::auth",
            scheme = "NimbusHmac",
            "session token missing, falling back to Nimbus HMAC"
        );
        crate::auth::apply_hmac_headers(headers, &self.access_key, &self.secret_key, payload)
    }
}

#[derive(Clone, Copy)]
enum Scope<'a> {
    Profile(&'a str),
    Global,
}

impl<'a> Scope<'a> {
    fn candidate_keys(&self, suffix: &str) -> Vec<String> {
        match self {
            Scope::Profile(name) => vec![
                format!("{PROFILE_PREFIX}_{name}_{suffix}"),
                format!("{GLOBAL_PREFIX}_{suffix}"),
            ],
            Scope::Global => vec![format!("{GLOBAL_PREFIX}_{suffix}")],
        }
    }
}

impl fmt::Display for Scope<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Scope::Profile(name) => write!(f, "profile:{name}"),
            Scope::Global => write!(f, "global"),
        }
    }
}

struct Redacted<'a>(&'a str);

impl fmt::Display for Redacted<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.0;
        if value.is_empty() {
            return write!(f, "***");
        }
        let visible = value.chars().take(4).collect::<String>();
        write!(f, "{}***", visible)
    }
}

fn normalize_profile(value: &str) -> Result<String, AuthError> {
    let normalized = value.trim().to_ascii_uppercase();
    if normalized.is_empty() {
        return Err(AuthError::InvalidProfile {
            profile: value.to_string(),
            reason: "profile name cannot be empty".to_string(),
        });
    }
    if normalized.len() > 16 {
        return Err(AuthError::InvalidProfile {
            profile: value.to_string(),
            reason: "profile name exceeds 16 characters".to_string(),
        });
    }
    let valid = normalized
        .chars()
        .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_');
    if !valid {
        return Err(AuthError::InvalidProfile {
            profile: value.to_string(),
            reason: "profile name must match ^[A-Z0-9_]{1,16}$".to_string(),
        });
    }
    Ok(normalized)
}

fn validate_access_key(value: &str) -> Result<(), &'static str> {
    if value.len() != 26 {
        return Err("value must be exactly 26 characters");
    }
    if !value
        .chars()
        .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
    {
        return Err("value must contain only uppercase A-Z or digits 0-9");
    }
    Ok(())
}

fn validate_secret_key(value: &str) -> Result<(), &'static str> {
    if value.len() != 64 {
        return Err("value must be exactly 64 characters");
    }
    if !value
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        return Err("value must be URL-safe base64 without padding");
    }
    Ok(())
}

fn validate_region(value: &str) -> Result<(), &'static str> {
    let mut parts = value.split('-');
    let first = parts.next().ok_or("value must include a region prefix")?;
    let second = parts.next().ok_or("value must include a region body")?;
    let third = parts.next().ok_or("value must include a numeric suffix")?;
    if parts.next().is_some() {
        return Err("value must follow <prefix>-<body>-<digits>");
    }
    if !first.chars().all(|c| c.is_ascii_lowercase()) || first.is_empty() {
        return Err("region prefix must be lowercase letters");
    }
    if !second.chars().all(|c| c.is_ascii_lowercase()) || second.is_empty() {
        return Err("region body must be lowercase letters");
    }
    if !(1..=2).contains(&third.len()) {
        return Err("region suffix must be 1 or 2 digits");
    }
    if !third.chars().all(|c| c.is_ascii_digit()) {
        return Err("region suffix must be numeric");
    }
    Ok(())
}
