//! Generated types – do not edit by hand.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum BootDecision {
    #[serde(rename = "offer")]
    Offer,
    #[serde(rename = "ignore")]
    Ignore,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BootLookupPayload {
    pub mac: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BootLookupResponse {
    pub decision: BootDecision,
    pub status: BootRegistryStatus,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "bootScriptUrl"
    )]
    pub boot_script_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bootImage")]
    pub boot_image: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "pendingExpiresAt"
    )]
    pub pending_expires_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BootRegistryEntry {
    pub mac: String,
    pub status: BootRegistryStatus,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bootImage")]
    pub boot_image: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "bootScriptUrl"
    )]
    pub boot_script_url: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "pendingExpiresAt"
    )]
    pub pending_expires_at: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "lastSeenAt"
    )]
    pub last_seen_at: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum BootRegistryStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "pending")]
    Pending,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BootRegistryUpsertPayload {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bootImage")]
    pub boot_image: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "bootScriptUrl"
    )]
    pub boot_script_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "pendingTtlSeconds"
    )]
    pub pending_ttl_seconds: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BootstrapCredentialsBody {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ttlSecs")]
    pub ttl_secs: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BootstrapCredentialsResponse {
    pub token: String,
    #[serde(rename = "expiresAt")]
    pub expires_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateNetworkPayload {
    pub name: String,
    pub hosts: CreateNetworkPayloadHostsList,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<NetworkPolicyPayload>,
}

pub type CreateNetworkPayloadHostsList = Vec<String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateNicPayload {
    #[serde(rename = "vmId")]
    pub vm_id: String,
    #[serde(rename = "networkId")]
    pub network_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hostId")]
    pub host_id: Option<String>,
    pub mac: String,
    #[serde(rename = "allowedPorts")]
    pub allowed_ports: CreateNicPayloadAllowedPortsList,
    #[serde(rename = "securityRules")]
    pub security_rules: CreateNicPayloadSecurityRulesList,
}

pub type CreateNicPayloadAllowedPortsList = Vec<PortRangePayload>;

pub type CreateNicPayloadSecurityRulesList = Vec<NicSecurityRulePayload>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateVmInterfacePayload {
    #[serde(rename = "networkId")]
    pub network_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateVmPayload {
    #[serde(rename = "shapeId")]
    pub shape_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interfaces: Option<CreateVmPayloadInterfacesList>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disks: Option<CreateVmPayloadDisksList>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<CreateVmPayloadTagsMap>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userData")]
    pub user_data: Option<String>,
}

pub type CreateVmPayloadDisksList = Vec<DiskPayload>;

pub type CreateVmPayloadInterfacesList = Vec<CreateVmInterfacePayload>;

pub type CreateVmPayloadTagsMap = BTreeMap<String, String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiskPayload {
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bus: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sizeBytes")]
    pub size_bytes: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FailPayload {
    pub error: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GpuInfoPayload {
    pub index: i32,
    pub r#type: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "availableVramKb"
    )]
    pub available_vram_kb: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "totalVramKb"
    )]
    pub total_vram_kb: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "totalVfs")]
    pub total_vfs: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "usedVfs")]
    pub used_vfs: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HeartbeatPayload {
    pub hostname: String,
    #[serde(rename = "totalMemory")]
    pub total_memory: i64,
    #[serde(rename = "usedMemory")]
    pub used_memory: i64,
    #[serde(rename = "physicalGpuCount")]
    pub physical_gpu_count: i32,
    pub gpus: HeartbeatPayloadGpusList,
    pub vms: HeartbeatPayloadVmsList,
    #[serde(rename = "cpuTopology")]
    pub cpu_topology: Value,
}

pub type HeartbeatPayloadGpusList = Vec<GpuInfoPayload>;

pub type HeartbeatPayloadVmsList = Vec<VmInfoPayload>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HostJobResponse {
    pub id: String,
    pub job: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HostShutdownResponse {
    #[serde(rename = "stopJobIds")]
    pub stop_job_ids: HostShutdownResponseStopJobIdsList,
}

pub type HostShutdownResponseStopJobIdsList = Vec<String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IdempotencyListResponse {
    pub records: IdempotencyListResponseRecordsList,
}

pub type IdempotencyListResponseRecordsList = Vec<IdempotencyRecord>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IdempotencyRecord {
    #[serde(rename = "principalUrn")]
    pub principal_urn: String,
    #[serde(rename = "endpointScope")]
    pub endpoint_scope: String,
    #[serde(rename = "idempotencyKey")]
    pub idempotency_key: String,
    #[serde(rename = "canonicalHash")]
    pub canonical_hash: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "responseStatus"
    )]
    pub response_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jobId")]
    pub job_id: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "lastSeenAt")]
    pub last_seen_at: String,
    #[serde(rename = "expiresAt")]
    pub expires_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InterfacePayload {
    pub iface: String,
    #[serde(rename = "hostId")]
    pub host_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkPolicyPayload {
    #[serde(rename = "allowedPorts")]
    pub allowed_ports: NetworkPolicyPayloadAllowedPortsList,
    #[serde(rename = "allowedMacs")]
    pub allowed_macs: NetworkPolicyPayloadAllowedMacsList,
}

pub type NetworkPolicyPayloadAllowedMacsList = Vec<String>;

pub type NetworkPolicyPayloadAllowedPortsList = Vec<PortRangePayload>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkProvisioningPayload {
    #[serde(rename = "hostId")]
    pub host_id: String,
    #[serde(rename = "networkId")]
    pub network_id: String,
    pub interface: WireGuardInterfacePayload,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NicSecurityRulePayload {
    pub direction: RuleDirectionPayload,
    pub protocol: RuleProtocolPayload,
    pub ports: NicSecurityRulePayloadPortsList,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "remoteCidr"
    )]
    pub remote_cidr: Option<String>,
    pub action: RuleActionPayload,
}

pub type NicSecurityRulePayloadPortsList = Vec<PortRangePayload>;

pub type OperationError = serde_json::Value;
pub type OperationHandle = nimbus_sdk_core::lro::OperationHandle;
pub type OperationMetadata = serde_json::Value;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PortRangePayload {
    pub from: i16,
    pub to: i16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrivateNetworkPayload {
    pub id: String,
    pub tenant: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hostIds")]
    pub host_ids: Option<PrivateNetworkPayloadHostIdsList>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<NetworkPolicyPayload>,
}

pub type PrivateNetworkPayloadHostIdsList = Vec<String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RotateAgentCredentialsRequest {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ttlSecs")]
    pub ttl_secs: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum RuleActionPayload {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "deny")]
    Deny,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum RuleDirectionPayload {
    #[serde(rename = "ingress")]
    Ingress,
    #[serde(rename = "egress")]
    Egress,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum RuleProtocolPayload {
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "udp")]
    Udp,
    #[serde(rename = "any")]
    Any,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SwitchPayload {
    pub name: String,
    #[serde(rename = "hostId")]
    pub host_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateNetworkPayload {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hosts: Option<UpdateNetworkPayloadHostsList>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<NetworkPolicyPayload>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "rotateKeys"
    )]
    pub rotate_keys: Option<bool>,
}

pub type UpdateNetworkPayloadHostsList = Vec<String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VirtualNicPayload {
    pub id: String,
    pub tenant: String,
    #[serde(rename = "networkId")]
    pub network_id: String,
    #[serde(rename = "vmId")]
    pub vm_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hostId")]
    pub host_id: Option<String>,
    pub mac: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "allowedPorts"
    )]
    pub allowed_ports: Option<VirtualNicPayloadAllowedPortsList>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "securityRules"
    )]
    pub security_rules: Option<VirtualNicPayloadSecurityRulesList>,
}

pub type VirtualNicPayloadAllowedPortsList = Vec<PortRangePayload>;

pub type VirtualNicPayloadSecurityRulesList = Vec<NicSecurityRulePayload>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VmCpuPinPayload {
    pub vcpu: i32,
    pub cpuset: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VmInfoPayload {
    pub pid: i32,
    pub id: String,
    pub memory: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<VmInfoPayloadTagsMap>,
}

pub type VmInfoPayloadTagsMap = BTreeMap<String, String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VmInterfacePayload {
    #[serde(rename = "networkId")]
    pub network_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tapName")]
    pub tap_name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VmMigrationRequestPayload {
    #[serde(rename = "destinationHostId")]
    pub destination_host_id: String,
    #[serde(rename = "destinationUri")]
    pub destination_uri: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "migrationUri"
    )]
    pub migration_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VmPayload {
    pub id: String,
    pub tenant: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hostId")]
    pub host_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<VmStatus>,
    #[serde(rename = "shapeId")]
    pub shape_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interfaces: Option<VmPayloadInterfacesList>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disks: Option<VmPayloadDisksList>,
    pub shape: VmResolvedShapePayload,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cpuUnits")]
    pub cpu_units: Option<i16>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sharedCoreMultiplier"
    )]
    pub shared_core_multiplier: Option<f32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sharedCoreOvercommitRatio"
    )]
    pub shared_core_overcommit_ratio: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<VmPayloadTagsMap>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userData")]
    pub user_data: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpuset: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vcpuPins")]
    pub vcpu_pins: Option<VmPayloadVcpuPinsList>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "numaNode")]
    pub numa_node: Option<i32>,
}

pub type VmPayloadDisksList = Vec<DiskPayload>;

pub type VmPayloadInterfacesList = Vec<VmInterfacePayload>;

pub type VmPayloadTagsMap = BTreeMap<String, String>;

pub type VmPayloadVcpuPinsList = Vec<VmCpuPinPayload>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VmResolvedShapePayload {
    pub memory: i64,
    pub vcpu: i32,
    #[serde(rename = "cpuUnits")]
    pub cpu_units: i16,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sharedCoreMultiplier"
    )]
    pub shared_core_multiplier: Option<f32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sharedCoreOvercommitRatio"
    )]
    pub shared_core_overcommit_ratio: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gpuType")]
    pub gpu_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gpuVramKb")]
    pub gpu_vram_kb: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum VmStatus {
    #[serde(rename = "stopped")]
    Stopped,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "stopping")]
    Stopping,
    #[serde(rename = "migrating")]
    Migrating,
    #[serde(rename = "deleting")]
    Deleting,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WireGuardInterfacePayload {
    pub name: String,
    pub address: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "listenPort"
    )]
    pub listen_port: Option<i16>,
    #[serde(rename = "privateKey")]
    pub private_key: String,
    #[serde(rename = "publicKey")]
    pub public_key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peers: Option<WireGuardInterfacePayloadPeersList>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routes: Option<WireGuardInterfacePayloadRoutesList>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bridge: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tap: Option<String>,
}

pub type WireGuardInterfacePayloadPeersList = Vec<WireGuardPeerPayload>;

pub type WireGuardInterfacePayloadRoutesList = Vec<String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WireGuardPeerPayload {
    #[serde(rename = "publicKey")]
    pub public_key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "allowedIps"
    )]
    pub allowed_ips: Option<WireGuardPeerPayloadAllowedIpsList>,
}

pub type WireGuardPeerPayloadAllowedIpsList = Vec<String>;
