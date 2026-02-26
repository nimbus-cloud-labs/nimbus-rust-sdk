//! Generated types – do not edit by hand.
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommandResponse {
    pub message: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateRecordRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    pub name: String,
    #[serde(rename = "recordType")]
    pub record_type: RecordTypePayload,
    pub ttl: i64,
    pub data: Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateZoneRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteRecordQuery {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteZoneQuery {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListRecordsResponse {
    pub records: ListRecordsResponseRecordsList,
}

pub type ListRecordsResponseRecordsList = Vec<RecordResponse>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListZonesResponse {
    pub zones: ListZonesResponseZonesList,
}

pub type ListZonesResponseZonesList = Vec<ZoneResponse>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RecordResponse {
    pub id: String,
    #[serde(rename = "zoneId")]
    pub zone_id: String,
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    pub name: String,
    #[serde(rename = "recordType")]
    pub record_type: RecordTypePayload,
    pub ttl: i32,
    pub data: Value,
    pub version: i64,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum RecordTypePayload {
    A,
    AAAA,
    CNAME,
    MX,
    NS,
    SOA,
    SRV,
    TXT,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateRecordRequest {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "recordType"
    )]
    pub record_type: Option<RecordTypePayload>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateZoneRequest {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ZoneResponse {
    pub id: String,
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    pub name: String,
    pub version: i64,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub buckets: ZoneResponseBucketsList,
}

pub type ZoneResponseBucketsList = Vec<i16>;
