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
pub struct ComputeClient {
    inner: NimbusClient,
}

impl ComputeClient {
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

    pub async fn attach_interface(
        &self,
        params: AttachInterfacePathParams<'_>,
        body: &InterfacePayload,
    ) -> Result<OperationHandle, SdkError> {
        let path_params = vec![("switch", params.switch.to_string())];
        let result = self
            .inner
            .invoke(&ATTACH_INTERFACE_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<OperationHandle>(result.body)
    }

    pub async fn bootstrap_agent_credentials(
        &self,
        params: BootstrapAgentCredentialsPathParams<'_>,
        body: &BootstrapCredentialsBody,
    ) -> Result<BootstrapCredentialsResponse, SdkError> {
        let path_params = vec![("host_id", params.host_id.to_string())];
        let result = self
            .inner
            .invoke(
                &BOOTSTRAP_AGENT_CREDENTIALS_SPEC,
                &path_params,
                Some(body),
                None,
            )
            .await?;
        self.inner
            .deserialize::<BootstrapCredentialsResponse>(result.body)
    }

    pub async fn claim_agent_job(
        &self,
        params: ClaimAgentJobPathParams<'_>,
    ) -> Result<HostJobResponse, SdkError> {
        let path_params = vec![("host_id", params.host_id.to_string())];
        let result = self
            .inner
            .invoke(&CLAIM_AGENT_JOB_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<HostJobResponse>(result.body)
    }

    pub async fn complete_agent_job(
        &self,
        params: CompleteAgentJobPathParams<'_>,
    ) -> Result<Value, SdkError> {
        let path_params = vec![
            ("host_id", params.host_id.to_string()),
            ("job_id", params.job_id.to_string()),
        ];
        let result = self
            .inner
            .invoke(&COMPLETE_AGENT_JOB_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<Value>(result.body)
    }

    pub async fn create_network(
        &self,
        body: &CreateNetworkPayload,
    ) -> Result<OperationHandle, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&CREATE_NETWORK_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<OperationHandle>(result.body)
    }

    pub async fn create_nic(&self, body: &CreateNicPayload) -> Result<OperationHandle, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&CREATE_NIC_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<OperationHandle>(result.body)
    }

    pub async fn create_switch(&self, body: &SwitchPayload) -> Result<OperationHandle, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&CREATE_SWITCH_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<OperationHandle>(result.body)
    }

    pub async fn create_vm(&self, body: &CreateVmPayload) -> Result<OperationHandle, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&CREATE_VM_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<OperationHandle>(result.body)
    }

    pub async fn delete_vm(
        &self,
        params: DeleteVmPathParams<'_>,
    ) -> Result<OperationHandle, SdkError> {
        let path_params = vec![("id", params.id.to_string())];
        let result = self
            .inner
            .invoke(&DELETE_VM_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<OperationHandle>(result.body)
    }

    pub async fn detach_interface(
        &self,
        params: DetachInterfacePathParams<'_>,
        body: &InterfacePayload,
    ) -> Result<OperationHandle, SdkError> {
        let path_params = vec![("switch", params.switch.to_string())];
        let result = self
            .inner
            .invoke(&DETACH_INTERFACE_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<OperationHandle>(result.body)
    }

    pub async fn fail_agent_job(
        &self,
        params: FailAgentJobPathParams<'_>,
        body: &FailPayload,
    ) -> Result<Value, SdkError> {
        let path_params = vec![
            ("host_id", params.host_id.to_string()),
            ("job_id", params.job_id.to_string()),
        ];
        let result = self
            .inner
            .invoke(&FAIL_AGENT_JOB_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<Value>(result.body)
    }

    pub async fn get_agent_metadata(
        &self,
        params: GetAgentMetadataPathParams<'_>,
    ) -> Result<Value, SdkError> {
        let path_params = vec![
            ("host_id", params.host_id.to_string()),
            ("path", params.path.to_string()),
        ];
        let result = self
            .inner
            .invoke(&GET_AGENT_METADATA_SPEC, &path_params, None, None)
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

    pub async fn heartbeat(&self, body: &HeartbeatPayload) -> Result<Value, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&HEARTBEAT_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<Value>(result.body)
    }

    pub async fn list_idempotency_records(&self) -> Result<IdempotencyListResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&LIST_IDEMPOTENCY_RECORDS_SPEC, &path_params, None, None)
            .await?;
        self.inner
            .deserialize::<IdempotencyListResponse>(result.body)
    }

    pub async fn list_networks(&self) -> Result<Value, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&LIST_NETWORKS_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<Value>(result.body)
    }

    pub fn paginate_list_networks(&self) -> Paginator<'_, Value> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        self.inner.paginator(&LIST_NETWORKS_SPEC, path_params)
    }

    pub async fn list_nics(&self) -> Result<Value, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&LIST_NICS_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<Value>(result.body)
    }

    pub fn paginate_list_nics(&self) -> Paginator<'_, Value> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        self.inner.paginator(&LIST_NICS_SPEC, path_params)
    }

    pub async fn migrate_vm(
        &self,
        params: MigrateVmPathParams<'_>,
        body: &VmMigrationRequestPayload,
    ) -> Result<OperationHandle, SdkError> {
        let path_params = vec![("id", params.id.to_string())];
        let result = self
            .inner
            .invoke(&MIGRATE_VM_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<OperationHandle>(result.body)
    }

    pub async fn report_agent_status(
        &self,
        params: ReportAgentStatusPathParams<'_>,
        body: &HeartbeatPayload,
    ) -> Result<Value, SdkError> {
        let path_params = vec![("host_id", params.host_id.to_string())];
        let result = self
            .inner
            .invoke(&REPORT_AGENT_STATUS_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<Value>(result.body)
    }

    pub async fn rotate_agent_credentials(
        &self,
        params: RotateAgentCredentialsPathParams<'_>,
        body: &RotateAgentCredentialsRequest,
    ) -> Result<Value, SdkError> {
        let path_params = vec![("host_id", params.host_id.to_string())];
        let result = self
            .inner
            .invoke(
                &ROTATE_AGENT_CREDENTIALS_SPEC,
                &path_params,
                Some(body),
                None,
            )
            .await?;
        self.inner.deserialize::<Value>(result.body)
    }

    pub async fn shutdown_host(
        &self,
        params: ShutdownHostPathParams<'_>,
    ) -> Result<HostShutdownResponse, SdkError> {
        let path_params = vec![("host_id", params.host_id.to_string())];
        let result = self
            .inner
            .invoke(&SHUTDOWN_HOST_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<HostShutdownResponse>(result.body)
    }

    pub async fn start_vm(
        &self,
        params: StartVmPathParams<'_>,
    ) -> Result<OperationHandle, SdkError> {
        let path_params = vec![("id", params.id.to_string())];
        let result = self
            .inner
            .invoke(&START_VM_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<OperationHandle>(result.body)
    }

    pub async fn stop_vm(&self, params: StopVmPathParams<'_>) -> Result<OperationHandle, SdkError> {
        let path_params = vec![("id", params.id.to_string())];
        let result = self
            .inner
            .invoke(&STOP_VM_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<OperationHandle>(result.body)
    }

    pub async fn update_network(
        &self,
        params: UpdateNetworkPathParams<'_>,
        body: &UpdateNetworkPayload,
    ) -> Result<OperationHandle, SdkError> {
        let path_params = vec![("network_id", params.network_id.to_string())];
        let result = self
            .inner
            .invoke(&UPDATE_NETWORK_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<OperationHandle>(result.body)
    }

    pub fn waiter(&self) -> LroWaiter {
        self.inner.waiter()
    }
}

#[derive(Clone, Debug)]
pub struct AttachInterfacePathParams<'a> {
    pub switch: &'a str,
}

#[derive(Clone, Debug)]
pub struct BootstrapAgentCredentialsPathParams<'a> {
    pub host_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct ClaimAgentJobPathParams<'a> {
    pub host_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct CompleteAgentJobPathParams<'a> {
    pub host_id: &'a str,
    pub job_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct DeleteVmPathParams<'a> {
    pub id: &'a str,
}

#[derive(Clone, Debug)]
pub struct DetachInterfacePathParams<'a> {
    pub switch: &'a str,
}

#[derive(Clone, Debug)]
pub struct FailAgentJobPathParams<'a> {
    pub host_id: &'a str,
    pub job_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct GetAgentMetadataPathParams<'a> {
    pub host_id: &'a str,
    pub path: &'a str,
}

#[derive(Clone, Debug)]
pub struct MigrateVmPathParams<'a> {
    pub id: &'a str,
}

#[derive(Clone, Debug)]
pub struct ReportAgentStatusPathParams<'a> {
    pub host_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct RotateAgentCredentialsPathParams<'a> {
    pub host_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct ShutdownHostPathParams<'a> {
    pub host_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct StartVmPathParams<'a> {
    pub id: &'a str,
}

#[derive(Clone, Debug)]
pub struct StopVmPathParams<'a> {
    pub id: &'a str,
}

#[derive(Clone, Debug)]
pub struct UpdateNetworkPathParams<'a> {
    pub network_id: &'a str,
}

const ATTACH_INTERFACE_SPEC: OperationSpec = OperationSpec {
    name: "AttachInterface",
    method: SdkHttpMethod::Post,
    uri: "/switches/{switch}/attach",
    success_code: 202,
    idempotent: true,
    pagination: None,
    lro: true,
};

const BOOTSTRAP_AGENT_CREDENTIALS_SPEC: OperationSpec = OperationSpec {
    name: "BootstrapAgentCredentials",
    method: SdkHttpMethod::Post,
    uri: "/agents/{host_id}/credentials/bootstrap",
    success_code: 200,
    idempotent: false,
    pagination: None,
    lro: false,
};

const CLAIM_AGENT_JOB_SPEC: OperationSpec = OperationSpec {
    name: "ClaimAgentJob",
    method: SdkHttpMethod::Post,
    uri: "/agents/{host_id}/jobs/next",
    success_code: 200,
    idempotent: true,
    pagination: None,
    lro: false,
};

const COMPLETE_AGENT_JOB_SPEC: OperationSpec = OperationSpec {
    name: "CompleteAgentJob",
    method: SdkHttpMethod::Post,
    uri: "/agents/{host_id}/jobs/{job_id}/complete",
    success_code: 200,
    idempotent: true,
    pagination: None,
    lro: false,
};

const CREATE_NETWORK_SPEC: OperationSpec = OperationSpec {
    name: "CreateNetwork",
    method: SdkHttpMethod::Post,
    uri: "/networks",
    success_code: 202,
    idempotent: true,
    pagination: None,
    lro: true,
};

const CREATE_NIC_SPEC: OperationSpec = OperationSpec {
    name: "CreateNic",
    method: SdkHttpMethod::Post,
    uri: "/nics",
    success_code: 202,
    idempotent: true,
    pagination: None,
    lro: true,
};

const CREATE_SWITCH_SPEC: OperationSpec = OperationSpec {
    name: "CreateSwitch",
    method: SdkHttpMethod::Post,
    uri: "/switches",
    success_code: 202,
    idempotent: true,
    pagination: None,
    lro: true,
};

const CREATE_VM_SPEC: OperationSpec = OperationSpec {
    name: "CreateVm",
    method: SdkHttpMethod::Post,
    uri: "/vms",
    success_code: 202,
    idempotent: true,
    pagination: None,
    lro: true,
};

const DELETE_VM_SPEC: OperationSpec = OperationSpec {
    name: "DeleteVm",
    method: SdkHttpMethod::Delete,
    uri: "/vms/{id}",
    success_code: 202,
    idempotent: true,
    pagination: None,
    lro: true,
};

const DETACH_INTERFACE_SPEC: OperationSpec = OperationSpec {
    name: "DetachInterface",
    method: SdkHttpMethod::Post,
    uri: "/switches/{switch}/detach",
    success_code: 202,
    idempotent: true,
    pagination: None,
    lro: true,
};

const FAIL_AGENT_JOB_SPEC: OperationSpec = OperationSpec {
    name: "FailAgentJob",
    method: SdkHttpMethod::Post,
    uri: "/agents/{host_id}/jobs/{job_id}/fail",
    success_code: 200,
    idempotent: true,
    pagination: None,
    lro: false,
};

const GET_AGENT_METADATA_SPEC: OperationSpec = OperationSpec {
    name: "GetAgentMetadata",
    method: SdkHttpMethod::Get,
    uri: "/agents/{host_id}/metadata/{*path}",
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

const HEARTBEAT_SPEC: OperationSpec = OperationSpec {
    name: "Heartbeat",
    method: SdkHttpMethod::Post,
    uri: "/heartbeat",
    success_code: 200,
    idempotent: false,
    pagination: None,
    lro: false,
};

const LIST_IDEMPOTENCY_RECORDS_SPEC: OperationSpec = OperationSpec {
    name: "ListIdempotencyRecords",
    method: SdkHttpMethod::Get,
    uri: "/internal/idempotency",
    success_code: 200,
    idempotent: false,
    pagination: None,
    lro: false,
};

const LIST_NETWORKS_SPEC: OperationSpec = OperationSpec {
    name: "ListNetworks",
    method: SdkHttpMethod::Get,
    uri: "/networks",
    success_code: 200,
    idempotent: false,
    pagination: Some(PaginationSpec {
        request_header: "Range",
        response_header: "Content-Range",
    }),
    lro: false,
};

const LIST_NICS_SPEC: OperationSpec = OperationSpec {
    name: "ListNics",
    method: SdkHttpMethod::Get,
    uri: "/nics",
    success_code: 200,
    idempotent: false,
    pagination: Some(PaginationSpec {
        request_header: "Range",
        response_header: "Content-Range",
    }),
    lro: false,
};

const MIGRATE_VM_SPEC: OperationSpec = OperationSpec {
    name: "MigrateVm",
    method: SdkHttpMethod::Post,
    uri: "/vms/{id}/migrate",
    success_code: 202,
    idempotent: true,
    pagination: None,
    lro: true,
};

const REPORT_AGENT_STATUS_SPEC: OperationSpec = OperationSpec {
    name: "ReportAgentStatus",
    method: SdkHttpMethod::Post,
    uri: "/agents/{host_id}/status",
    success_code: 200,
    idempotent: false,
    pagination: None,
    lro: false,
};

const ROTATE_AGENT_CREDENTIALS_SPEC: OperationSpec = OperationSpec {
    name: "RotateAgentCredentials",
    method: SdkHttpMethod::Put,
    uri: "/agents/{host_id}/credentials",
    success_code: 202,
    idempotent: false,
    pagination: None,
    lro: true,
};

const SHUTDOWN_HOST_SPEC: OperationSpec = OperationSpec {
    name: "ShutdownHost",
    method: SdkHttpMethod::Post,
    uri: "/hosts/{host_id}/shutdown",
    success_code: 202,
    idempotent: true,
    pagination: None,
    lro: true,
};

const START_VM_SPEC: OperationSpec = OperationSpec {
    name: "StartVm",
    method: SdkHttpMethod::Post,
    uri: "/vms/{id}/start",
    success_code: 202,
    idempotent: true,
    pagination: None,
    lro: true,
};

const STOP_VM_SPEC: OperationSpec = OperationSpec {
    name: "StopVm",
    method: SdkHttpMethod::Post,
    uri: "/vms/{id}/stop",
    success_code: 202,
    idempotent: true,
    pagination: None,
    lro: true,
};

const UPDATE_NETWORK_SPEC: OperationSpec = OperationSpec {
    name: "UpdateNetwork",
    method: SdkHttpMethod::Put,
    uri: "/networks/{network_id}",
    success_code: 202,
    idempotent: true,
    pagination: None,
    lro: true,
};
