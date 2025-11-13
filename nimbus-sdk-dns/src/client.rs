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
pub struct DnsClient {
    inner: NimbusClient,
}

impl DnsClient {
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

    pub async fn create_record(
        &self,
        params: CreateRecordPathParams<'_>,
        body: &CreateRecordRequest,
    ) -> Result<RecordResponse, SdkError> {
        let path_params = vec![("zone_id", params.zone_id.to_string())];
        let result = self
            .inner
            .invoke(&CREATE_RECORD_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<RecordResponse>(result.body)
    }

    pub async fn create_zone(&self, body: &CreateZoneRequest) -> Result<ZoneResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&CREATE_ZONE_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<ZoneResponse>(result.body)
    }

    pub async fn delete_record(
        &self,
        params: DeleteRecordPathParams<'_>,
        body: &DeleteRecordQuery,
    ) -> Result<RecordResponse, SdkError> {
        let path_params = vec![("record_id", params.record_id.to_string())];
        let result = self
            .inner
            .invoke(&DELETE_RECORD_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<RecordResponse>(result.body)
    }

    pub async fn delete_zone(
        &self,
        params: DeleteZonePathParams<'_>,
        body: &DeleteZoneQuery,
    ) -> Result<ZoneResponse, SdkError> {
        let path_params = vec![("zone_id", params.zone_id.to_string())];
        let result = self
            .inner
            .invoke(&DELETE_ZONE_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<ZoneResponse>(result.body)
    }

    pub async fn get_health(&self) -> Result<Value, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&GET_HEALTH_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<Value>(result.body)
    }

    pub async fn get_metrics(&self) -> Result<Value, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&GET_METRICS_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<Value>(result.body)
    }

    pub async fn list_records(&self) -> Result<ListRecordsResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&LIST_RECORDS_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<ListRecordsResponse>(result.body)
    }

    pub fn paginate_list_records(&self) -> Paginator<'_, ListRecordsResponse> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        self.inner.paginator(&LIST_RECORDS_SPEC, path_params)
    }

    pub async fn list_zone_records(
        &self,
        params: ListZoneRecordsPathParams<'_>,
    ) -> Result<ListRecordsResponse, SdkError> {
        let path_params = vec![("zone_id", params.zone_id.to_string())];
        let result = self
            .inner
            .invoke(&LIST_ZONE_RECORDS_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<ListRecordsResponse>(result.body)
    }

    pub fn paginate_list_zone_records(
        &self,
        params: ListZoneRecordsPathParams<'_>,
    ) -> Paginator<'_, ListRecordsResponse> {
        let path_params = vec![("zone_id", params.zone_id.to_string())];
        self.inner.paginator(&LIST_ZONE_RECORDS_SPEC, path_params)
    }

    pub async fn list_zones(&self) -> Result<ListZonesResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&LIST_ZONES_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<ListZonesResponse>(result.body)
    }

    pub fn paginate_list_zones(&self) -> Paginator<'_, ListZonesResponse> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        self.inner.paginator(&LIST_ZONES_SPEC, path_params)
    }

    pub async fn reload_runtime(&self) -> Result<CommandResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&RELOAD_RUNTIME_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<CommandResponse>(result.body)
    }

    pub async fn reload_zone(
        &self,
        params: ReloadZonePathParams<'_>,
    ) -> Result<CommandResponse, SdkError> {
        let path_params = vec![("zone_id", params.zone_id.to_string())];
        let result = self
            .inner
            .invoke(&RELOAD_ZONE_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<CommandResponse>(result.body)
    }

    pub async fn schedule_runtime_sync(&self) -> Result<CommandResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&SCHEDULE_RUNTIME_SYNC_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<CommandResponse>(result.body)
    }

    pub async fn schedule_zone_sync(
        &self,
        params: ScheduleZoneSyncPathParams<'_>,
    ) -> Result<CommandResponse, SdkError> {
        let path_params = vec![("zone_id", params.zone_id.to_string())];
        let result = self
            .inner
            .invoke(&SCHEDULE_ZONE_SYNC_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<CommandResponse>(result.body)
    }

    pub async fn update_record(
        &self,
        params: UpdateRecordPathParams<'_>,
        body: &UpdateRecordRequest,
    ) -> Result<RecordResponse, SdkError> {
        let path_params = vec![("record_id", params.record_id.to_string())];
        let result = self
            .inner
            .invoke(&UPDATE_RECORD_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<RecordResponse>(result.body)
    }

    pub async fn update_zone(
        &self,
        params: UpdateZonePathParams<'_>,
        body: &UpdateZoneRequest,
    ) -> Result<ZoneResponse, SdkError> {
        let path_params = vec![("zone_id", params.zone_id.to_string())];
        let result = self
            .inner
            .invoke(&UPDATE_ZONE_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<ZoneResponse>(result.body)
    }

    pub fn waiter(&self) -> LroWaiter {
        self.inner.waiter()
    }
}

#[derive(Clone, Debug)]
pub struct CreateRecordPathParams<'a> {
    pub zone_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct DeleteRecordPathParams<'a> {
    pub record_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct DeleteZonePathParams<'a> {
    pub zone_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct ListZoneRecordsPathParams<'a> {
    pub zone_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct ReloadZonePathParams<'a> {
    pub zone_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct ScheduleZoneSyncPathParams<'a> {
    pub zone_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct UpdateRecordPathParams<'a> {
    pub record_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct UpdateZonePathParams<'a> {
    pub zone_id: &'a str,
}

const CREATE_RECORD_SPEC: OperationSpec = OperationSpec {
    name: "CreateRecord",
    method: SdkHttpMethod::Post,
    uri: "/zones/{zone_id}/records",
    success_code: 201,
    idempotent: false,
    pagination: None,
    lro: false,
};

const CREATE_ZONE_SPEC: OperationSpec = OperationSpec {
    name: "CreateZone",
    method: SdkHttpMethod::Post,
    uri: "/zones",
    success_code: 201,
    idempotent: false,
    pagination: None,
    lro: false,
};

const DELETE_RECORD_SPEC: OperationSpec = OperationSpec {
    name: "DeleteRecord",
    method: SdkHttpMethod::Delete,
    uri: "/records/{record_id}",
    success_code: 200,
    idempotent: false,
    pagination: None,
    lro: false,
};

const DELETE_ZONE_SPEC: OperationSpec = OperationSpec {
    name: "DeleteZone",
    method: SdkHttpMethod::Delete,
    uri: "/zones/{zone_id}",
    success_code: 200,
    idempotent: false,
    pagination: None,
    lro: false,
};

const GET_HEALTH_SPEC: OperationSpec = OperationSpec {
    name: "GetHealth",
    method: SdkHttpMethod::Get,
    uri: "/health",
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

const LIST_RECORDS_SPEC: OperationSpec = OperationSpec {
    name: "ListRecords",
    method: SdkHttpMethod::Get,
    uri: "/records",
    success_code: 200,
    idempotent: false,
    pagination: Some(PaginationSpec {
        request_header: "Range",
        response_header: "Content-Range",
    }),
    lro: false,
};

const LIST_ZONE_RECORDS_SPEC: OperationSpec = OperationSpec {
    name: "ListZoneRecords",
    method: SdkHttpMethod::Get,
    uri: "/zones/{zone_id}/records",
    success_code: 200,
    idempotent: false,
    pagination: Some(PaginationSpec {
        request_header: "Range",
        response_header: "Content-Range",
    }),
    lro: false,
};

const LIST_ZONES_SPEC: OperationSpec = OperationSpec {
    name: "ListZones",
    method: SdkHttpMethod::Get,
    uri: "/zones",
    success_code: 200,
    idempotent: false,
    pagination: Some(PaginationSpec {
        request_header: "Range",
        response_header: "Content-Range",
    }),
    lro: false,
};

const RELOAD_RUNTIME_SPEC: OperationSpec = OperationSpec {
    name: "ReloadRuntime",
    method: SdkHttpMethod::Post,
    uri: "/runtime/reload",
    success_code: 200,
    idempotent: false,
    pagination: None,
    lro: false,
};

const RELOAD_ZONE_SPEC: OperationSpec = OperationSpec {
    name: "ReloadZone",
    method: SdkHttpMethod::Post,
    uri: "/zones/{zone_id}/reload",
    success_code: 200,
    idempotent: false,
    pagination: None,
    lro: false,
};

const SCHEDULE_RUNTIME_SYNC_SPEC: OperationSpec = OperationSpec {
    name: "ScheduleRuntimeSync",
    method: SdkHttpMethod::Post,
    uri: "/runtime/sync",
    success_code: 202,
    idempotent: false,
    pagination: None,
    lro: true,
};

const SCHEDULE_ZONE_SYNC_SPEC: OperationSpec = OperationSpec {
    name: "ScheduleZoneSync",
    method: SdkHttpMethod::Post,
    uri: "/zones/{zone_id}/sync",
    success_code: 202,
    idempotent: false,
    pagination: None,
    lro: true,
};

const UPDATE_RECORD_SPEC: OperationSpec = OperationSpec {
    name: "UpdateRecord",
    method: SdkHttpMethod::Put,
    uri: "/records/{record_id}",
    success_code: 200,
    idempotent: false,
    pagination: None,
    lro: false,
};

const UPDATE_ZONE_SPEC: OperationSpec = OperationSpec {
    name: "UpdateZone",
    method: SdkHttpMethod::Put,
    uri: "/zones/{zone_id}",
    success_code: 200,
    idempotent: false,
    pagination: None,
    lro: false,
};
