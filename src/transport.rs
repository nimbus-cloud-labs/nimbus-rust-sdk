use std::sync::Arc;

use async_trait::async_trait;
use http::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Client;
use serde_json::Value;
use thiserror::Error;
use url::Url;

#[derive(Clone, Copy, Debug)]
pub enum SdkHttpMethod {
    Delete,
    Get,
    Patch,
    Post,
    Put,
}

impl SdkHttpMethod {
    pub(crate) fn as_reqwest(&self) -> reqwest::Method {
        match self {
            Self::Delete => reqwest::Method::DELETE,
            Self::Get => reqwest::Method::GET,
            Self::Patch => reqwest::Method::PATCH,
            Self::Post => reqwest::Method::POST,
            Self::Put => reqwest::Method::PUT,
        }
    }
}

#[derive(Debug, Error)]
pub enum TransportError {
    #[error("failed to build request URL: {0}")]
    InvalidUrl(#[from] url::ParseError),
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
}

#[async_trait]
pub trait Transport: Send + Sync {
    async fn execute(&self, request: TransportRequest)
        -> Result<TransportResponse, TransportError>;
}

#[derive(Debug)]
pub struct TransportRequest {
    pub method: SdkHttpMethod,
    pub path: String,
    pub headers: HeaderMap,
    pub body: Option<Value>,
}

#[derive(Debug)]
pub struct TransportResponse {
    pub status: u16,
    pub headers: HeaderMap,
    pub body: Value,
}

#[derive(Clone)]
pub struct ReqwestTransport {
    base_url: Arc<Url>,
    client: Client,
}

impl ReqwestTransport {
    pub fn new(base_url: Url) -> Self {
        Self {
            client: Client::new(),
            base_url: Arc::new(base_url),
        }
    }
}

#[async_trait]
impl Transport for ReqwestTransport {
    async fn execute(
        &self,
        request: TransportRequest,
    ) -> Result<TransportResponse, TransportError> {
        let url = self.base_url.join(&request.path)?;
        let mut builder = self.client.request(request.method.as_reqwest(), url);
        for (name, value) in request.headers.iter() {
            builder = builder.header(name, value);
        }
        if let Some(body) = request.body {
            builder = builder.json(&body);
        }
        let response = builder.send().await?;
        let status = response.status().as_u16();
        let headers = response.headers().clone();
        let body = response.json().await.unwrap_or(Value::Null);
        Ok(TransportResponse {
            status,
            headers,
            body,
        })
    }
}

pub fn header_map() -> HeaderMap {
    HeaderMap::new()
}

pub fn header(name: &str, value: &str) -> (HeaderName, HeaderValue) {
    (
        HeaderName::from_bytes(name.as_bytes()).expect("invalid header name"),
        HeaderValue::from_str(value).expect("invalid header value"),
    )
}
