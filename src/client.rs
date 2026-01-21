use std::collections::HashSet;
use std::sync::Arc;

use http::{HeaderMap, HeaderValue};
use serde::de::DeserializeOwned;
use serde_json::Value;
use thiserror::Error;
use url::Url;

use crate::auth::AuthTokenProvider;
use crate::error::SdkError;
use crate::idempotency::{self, IdempotencyTokenProvider};
use crate::lro::{LroWaiter, OperationStatusClient};
use crate::paginator::Paginator;
use crate::transport::{ReqwestTransport, SdkHttpMethod, Transport, TransportRequest};

#[derive(Clone)]
pub struct NimbusClient {
    inner: Arc<Inner>,
}

struct Inner {
    transport: Arc<dyn Transport>,
    auth: Arc<dyn AuthTokenProvider>,
    idempotency: Arc<dyn IdempotencyTokenProvider>,
    lro: Arc<dyn OperationStatusClient>,
}

pub struct OperationSpec {
    pub name: &'static str,
    pub method: SdkHttpMethod,
    pub uri: &'static str,
    pub success_code: u16,
    pub additional_success_responses: &'static [AdditionalSuccessResponseSpec],
    pub idempotent: bool,
    pub pagination: Option<PaginationSpec>,
    pub lro: bool,
}

#[derive(Clone, Copy)]
pub struct AdditionalSuccessResponseSpec {
    pub code: u16,
    pub has_body: bool,
}

#[derive(Clone, Copy)]
pub struct PaginationSpec {
    pub request_header: &'static str,
    pub response_header: &'static str,
}

pub struct OperationResult {
    pub status: u16,
    pub body: Value,
    pub next_cursor: Option<String>,
}

#[derive(Default)]
pub struct SdkConfigBuilder {
    endpoint: Option<String>,
    transport: Option<Arc<dyn Transport>>,
    auth: Option<Arc<dyn AuthTokenProvider>>,
    idempotency: Option<Arc<dyn IdempotencyTokenProvider>>,
    lro: Option<Arc<dyn OperationStatusClient>>,
}

pub struct SdkConfig {
    pub(crate) transport: Arc<dyn Transport>,
    pub(crate) auth: Arc<dyn AuthTokenProvider>,
    pub(crate) idempotency: Arc<dyn IdempotencyTokenProvider>,
    pub(crate) lro: Arc<dyn OperationStatusClient>,
}

impl NimbusClient {
    pub fn builder() -> SdkConfigBuilder {
        SdkConfigBuilder::default()
    }

    pub fn new(config: SdkConfig) -> Self {
        let inner = Inner {
            transport: config.transport,
            auth: config.auth,
            idempotency: config.idempotency,
            lro: config.lro,
        };
        Self {
            inner: Arc::new(inner),
        }
    }

    pub async fn invoke(
        &self,
        spec: &'static OperationSpec,
        path_params: &[(&'static str, String)],
        body: Option<&Value>,
        cursor: Option<&str>,
    ) -> Result<OperationResult, SdkError> {
        let path = Self::render_path(spec.uri, path_params)?;
        let mut headers = HeaderMap::new();

        if spec.idempotent {
            let token = self.inner.idempotency.next_token();
            headers.insert(
                "Idempotency-Key",
                HeaderValue::from_str(&token).map_err(SdkError::Header)?,
            );
        }

        if let Some(pagination) = spec.pagination.as_ref() {
            if let Some(cursor_value) = cursor {
                headers.insert(
                    pagination.request_header,
                    HeaderValue::from_str(cursor_value).map_err(SdkError::Header)?,
                );
            }
        }

        let payload_bytes = match body {
            Some(value) => serde_json::to_vec(value).map_err(SdkError::Json)?,
            None => Vec::new(),
        };
        self.inner.auth.apply(&mut headers, &payload_bytes).await?;

        let request = TransportRequest {
            method: spec.method,
            path,
            headers,
            body: body.cloned(),
        };

        let response = self.inner.transport.execute(request).await?;
        let accepted = self.acceptable_status(spec, response.status);
        if !accepted {
            return Err(SdkError::UnexpectedStatus {
                operation: spec.name,
                status: response.status,
            });
        }

        let next_cursor = spec
            .pagination
            .as_ref()
            .and_then(|p| response.headers.get(p.response_header))
            .and_then(|value| value.to_str().ok())
            .map(|value| value.to_string());

        Ok(OperationResult {
            status: response.status,
            body: response.body,
            next_cursor,
        })
    }

    pub fn paginator<'client, T>(
        &'client self,
        spec: &'static OperationSpec,
        path_params: Vec<(&'static str, String)>,
    ) -> Paginator<'client, T>
    where
        T: DeserializeOwned,
    {
        Paginator::new(self, spec, path_params)
    }

    pub fn waiter(&self) -> LroWaiter {
        LroWaiter::new(self.inner.lro.clone())
    }

    pub fn deserialize<T>(&self, value: Value) -> Result<T, SdkError>
    where
        T: DeserializeOwned,
    {
        Ok(serde_json::from_value(value)?)
    }

    fn render_path(
        template: &str,
        path_params: &[(&'static str, String)],
    ) -> Result<String, SdkError> {
        let mut rendered = template.to_string();
        let provided: HashSet<&str> = path_params.iter().map(|(k, _)| *k).collect();

        for capture in template.match_indices('{') {
            let start = capture.0;
            if let Some(end) = template[start..].find('}') {
                let placeholder = &template[start + 1..start + end];
                let name = placeholder.trim_start_matches('*');
                if !provided.contains(name) {
                    return Err(SdkError::InvalidPath(name.to_string()));
                }
            }
        }

        for (name, value) in path_params {
            rendered = rendered.replace(&format!("{{{}}}", name), value);
            rendered = rendered.replace(&format!("{{*{}}}", name), value);
        }

        Ok(rendered)
    }

    fn acceptable_status(&self, spec: &OperationSpec, status: u16) -> bool {
        if status == spec.success_code {
            return true;
        }
        if spec
            .additional_success_responses
            .iter()
            .any(|response| response.code == status)
        {
            return true;
        }
        if spec.pagination.is_some() && status == 206 {
            return true;
        }
        false
    }
}

impl SdkConfigBuilder {
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }

    pub fn transport(mut self, transport: Arc<dyn Transport>) -> Self {
        self.transport = Some(transport);
        self
    }

    pub fn auth(mut self, provider: Arc<dyn AuthTokenProvider>) -> Self {
        self.auth = Some(provider);
        self
    }

    pub fn idempotency(mut self, provider: Arc<dyn IdempotencyTokenProvider>) -> Self {
        self.idempotency = Some(provider);
        self
    }

    pub fn lro_client(mut self, client: Arc<dyn OperationStatusClient>) -> Self {
        self.lro = Some(client);
        self
    }

    pub fn build(self) -> Result<SdkConfig, BuildError> {
        let auth = self.auth.ok_or(BuildError::MissingAuth)?;

        let transport = match (self.transport, self.endpoint) {
            (Some(transport), _) => transport,
            (None, Some(endpoint)) => {
                let url = Url::parse(&endpoint).map_err(BuildError::InvalidEndpoint)?;
                Arc::new(ReqwestTransport::new(url)) as Arc<dyn Transport>
            }
            (None, None) => return Err(BuildError::MissingEndpoint),
        };

        let idempotency = self
            .idempotency
            .unwrap_or_else(|| idempotency::default_provider());
        let lro = self
            .lro
            .unwrap_or_else(|| Arc::new(crate::lro::NoopOperationStatusClient));

        Ok(SdkConfig {
            transport,
            auth,
            idempotency,
            lro,
        })
    }
}

#[derive(Debug, Error)]
pub enum BuildError {
    #[error("missing authentication provider")]
    MissingAuth,
    #[error("missing endpoint or custom transport")]
    MissingEndpoint,
    #[error("invalid endpoint: {0}")]
    InvalidEndpoint(url::ParseError),
}
