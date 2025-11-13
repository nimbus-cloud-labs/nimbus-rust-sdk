//! Generated client – do not edit by hand.
#![allow(clippy::all)]

use crate::types::*;
use nimbus_sdk_core::{
    client::{NimbusClient, OperationSpec, PaginationSpec, SdkConfig},
    error::SdkError,
    lro::LroWaiter,
    paginator::Paginator,
    SdkHttpMethod,
};
use serde_json::Value;

#[derive(Clone)]
pub struct LoadBalancerClient {
    inner: NimbusClient,
}

impl LoadBalancerClient {
    pub fn new(inner: NimbusClient) -> Self {
        Self { inner }
    }

    pub fn from_config(config: SdkConfig) -> Self {
        Self {
            inner: NimbusClient::new(config),
        }
    }

    pub fn inner(&self) -> &NimbusClient {
        &self.inner
    }

    /// Liveness probe for the load balancer management API.
    pub async fn get_health(&self) -> Result<HealthResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&GET_HEALTH_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<HealthResponse>(result.body)
    }

    /// Prometheus metrics endpoint used by operators.
    pub async fn get_metrics(&self) -> Result<Value, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&GET_METRICS_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<Value>(result.body)
    }

    /// Returns listener statuses. Requires the lb:read scope and honors Range pagination.
    pub async fn list_listeners(&self) -> Result<ListListenersResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&LIST_LISTENERS_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<ListListenersResponse>(result.body)
    }

    pub fn paginate_list_listeners(&self) -> Paginator<'_, ListListenersResponse> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        self.inner.paginator(&LIST_LISTENERS_SPEC, path_params)
    }

    /// Reloads listener definitions. Requires the lb:manage scope.
    pub async fn reload_listeners(&self) -> Result<CommandResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&RELOAD_LISTENERS_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<CommandResponse>(result.body)
    }

    /// Schedules reconciliation against the control plane. Requires the lb:manage scope.
    pub async fn schedule_sync(&self) -> Result<CommandResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&SCHEDULE_SYNC_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<CommandResponse>(result.body)
    }

    pub fn waiter(&self) -> LroWaiter {
        self.inner.waiter()
    }
}

const GET_HEALTH_SPEC: OperationSpec = OperationSpec {
    name: "GetHealth",
    method: SdkHttpMethod::Get,
    uri: "/healthz",
    success_code: 200,
    idempotent: false,
    pagination: None,
    lro: false,
};

const GET_METRICS_SPEC: OperationSpec = OperationSpec {
    name: "GetMetrics",
    method: SdkHttpMethod::Get,
    uri: "/metrics",
    success_code: 200,
    idempotent: false,
    pagination: None,
    lro: false,
};

const LIST_LISTENERS_SPEC: OperationSpec = OperationSpec {
    name: "ListListeners",
    method: SdkHttpMethod::Get,
    uri: "/listeners",
    success_code: 200,
    idempotent: false,
    pagination: Some(PaginationSpec {
        request_header: "Range",
        response_header: "Content-Range",
    }),
    lro: false,
};

const RELOAD_LISTENERS_SPEC: OperationSpec = OperationSpec {
    name: "ReloadListeners",
    method: SdkHttpMethod::Post,
    uri: "/listeners/reload",
    success_code: 200,
    idempotent: false,
    pagination: None,
    lro: false,
};

const SCHEDULE_SYNC_SPEC: OperationSpec = OperationSpec {
    name: "ScheduleSync",
    method: SdkHttpMethod::Post,
    uri: "/listeners/sync",
    success_code: 202,
    idempotent: false,
    pagination: None,
    lro: true,
};
