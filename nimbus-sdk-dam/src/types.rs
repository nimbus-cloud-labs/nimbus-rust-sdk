//! Generated types – do not edit by hand.
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssetDetailResponse {
    pub asset: AssetRecord,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "latestVersion"
    )]
    pub latest_version: Option<UploadedAssetVersion>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssetDownloadResponse {
    #[serde(rename = "downloadUrl")]
    pub download_url: String,
    #[serde(rename = "expiresAt")]
    pub expires_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssetFilterPayload {
    pub kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "collectionId"
    )]
    pub collection_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "albumId")]
    pub album_id: Option<String>,
    pub path: AssetFilterPayloadPathList,
}

pub type AssetFilterPayloadPathList = Vec<String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssetPrefixListResponse {
    pub items: AssetPrefixListResponseItemsList,
}

pub type AssetPrefixListResponseItemsList = Vec<AssetPrefixRecord>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssetPrefixRecord {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "prefixId")]
    pub prefix_id: String,
    pub prefix: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssetPrefixResponse {
    pub prefix: AssetPrefixRecord,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssetRecord {
    #[serde(rename = "assetId")]
    pub asset_id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "mediaType")]
    pub media_type: String,
    #[serde(rename = "lifecycleState")]
    pub lifecycle_state: String,
    pub metadata: AssetRecordMetadataList,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

pub type AssetRecordMetadataList = Vec<MetadataEntry>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssetRenditionListResponse {
    pub items: AssetRenditionListResponseItemsList,
}

pub type AssetRenditionListResponseItemsList = Vec<AssetRenditionRecord>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssetRenditionRecord {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "assetId")]
    pub asset_id: String,
    #[serde(rename = "renditionId")]
    pub rendition_id: String,
    #[serde(rename = "versionId")]
    pub version_id: String,
    pub profile: String,
    #[serde(rename = "pipelineRef")]
    pub pipeline_ref: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "pipelineRevision"
    )]
    pub pipeline_revision: Option<String>,
    #[serde(rename = "storageKey")]
    pub storage_key: String,
    pub status: String,
    pub metadata: Value,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "contentType"
    )]
    pub content_type: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "completedAt"
    )]
    pub completed_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssetSearchItem {
    #[serde(rename = "assetId")]
    pub asset_id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "mediaType")]
    pub media_type: String,
    #[serde(rename = "lifecycleState")]
    pub lifecycle_state: String,
    #[serde(rename = "bucketName")]
    pub bucket_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub score: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssetSearchRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
    pub filters: AssetSearchRequestFiltersList,
    pub sorts: AssetSearchRequestSortsList,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

pub type AssetSearchRequestFiltersList = Vec<AssetFilterPayload>;

pub type AssetSearchRequestSortsList = Vec<AssetSortPayload>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssetSearchResponse {
    pub total: i64,
    pub items: AssetSearchResponseItemsList,
}

pub type AssetSearchResponseItemsList = Vec<AssetSearchItem>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssetSortPayload {
    pub field: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssetVersionMetadataResponse {
    pub metadata: AssetVersionMetadataResponseMetadataList,
}

pub type AssetVersionMetadataResponseMetadataList = Vec<MetadataEntry>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BucketListResponse {
    pub items: BucketListResponseItemsList,
}

pub type BucketListResponseItemsList = Vec<BucketRecord>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BucketRecord {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "bucketName")]
    pub bucket_name: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CollectionListResponse {
    pub items: CollectionListResponseItemsList,
}

pub type CollectionListResponseItemsList = Vec<CollectionResponse>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CollectionMembershipChangeRequest {
    pub members: CollectionMembershipChangeRequestMembersList,
}

pub type CollectionMembershipChangeRequestMembersList = Vec<CollectionMembershipRequest>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CollectionMembershipListResponse {
    pub items: CollectionMembershipListResponseItemsList,
}

pub type CollectionMembershipListResponseItemsList = Vec<CollectionMembershipResponse>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CollectionMembershipRequest {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "membershipId"
    )]
    pub membership_id: Option<String>,
    #[serde(rename = "assetId")]
    pub asset_id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "renditionId"
    )]
    pub rendition_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "addedBy")]
    pub added_by: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CollectionMembershipResponse {
    #[serde(rename = "membershipId")]
    pub membership_id: String,
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "collectionId")]
    pub collection_id: String,
    #[serde(rename = "assetId")]
    pub asset_id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "renditionId"
    )]
    pub rendition_id: Option<String>,
    #[serde(rename = "addedAt")]
    pub added_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "addedBy")]
    pub added_by: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CollectionResponse {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "collectionId")]
    pub collection_id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "lockVersion")]
    pub lock_version: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompleteIngestionRequest {
    #[serde(rename = "uploadToken")]
    pub upload_token: String,
    #[serde(rename = "checksumAlgorithm")]
    pub checksum_algorithm: String,
    pub checksum: String,
    #[serde(rename = "sizeBytes")]
    pub size_bytes: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateAssetPrefixRequest {
    pub prefix: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateBucketRequest {
    #[serde(rename = "bucketName")]
    pub bucket_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateCollectionRequest {
    #[serde(rename = "collectionId")]
    pub collection_id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreatePipelineRequest {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "pipelinePrefix"
    )]
    pub pipeline_prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cdnHost")]
    pub cdn_host: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    pub renditions: CreatePipelineRequestRenditionsList,
    pub rules: CreatePipelineRequestRulesList,
}

pub type CreatePipelineRequestRenditionsList = Vec<PipelineRenditionRequest>;

pub type CreatePipelineRequestRulesList = Vec<PipelineRuleRequest>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateSmartAlbumRequest {
    #[serde(rename = "albumId")]
    pub album_id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub definition: Value,
    pub rules: CreateSmartAlbumRequestRulesList,
}

pub type CreateSmartAlbumRequestRulesList = Vec<SmartAlbumRuleRequest>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomMetadataRequest {
    #[serde(rename = "customMetadata")]
    pub custom_metadata: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndexSnapshotRequest {
    #[serde(rename = "snapshotId")]
    pub snapshot_id: String,
    #[serde(rename = "assetId")]
    pub asset_id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "renditionId"
    )]
    pub rendition_id: Option<String>,
    #[serde(rename = "indexScope")]
    pub index_scope: String,
    pub document: String,
    pub metadata: Value,
    #[serde(rename = "materializedAt")]
    pub materialized_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndexSnapshotResponse {
    #[serde(rename = "snapshotId")]
    pub snapshot_id: String,
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "assetId")]
    pub asset_id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "renditionId"
    )]
    pub rendition_id: Option<String>,
    #[serde(rename = "indexScope")]
    pub index_scope: String,
    pub document: String,
    pub metadata: Value,
    #[serde(rename = "materializedAt")]
    pub materialized_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IngestionRequest {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "mediaType")]
    pub media_type: String,
    #[serde(rename = "contentLength")]
    pub content_length: i64,
    #[serde(rename = "checksumAlgorithm")]
    pub checksum_algorithm: String,
    pub checksum: String,
    pub metadata: Value,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "pathPrefix"
    )]
    pub path_prefix: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "bucketName"
    )]
    pub bucket_name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IngestionResponse {
    pub asset: RegisteredAsset,
    pub version: PendingUploadVersion,
    pub upload: UploadTicket,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LifecycleExposureRequest {
    #[serde(rename = "pipelineRef")]
    pub pipeline_ref: String,
    #[serde(rename = "ruleSlug")]
    pub rule_slug: String,
    pub path: String,
    #[serde(rename = "contentType")]
    pub content_type: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "storageKey"
    )]
    pub storage_key: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "cachePolicy"
    )]
    pub cache_policy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<LifecycleExposureSchedule>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LifecycleExposureSchedule {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startsAt")]
    pub starts_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "expiresAt")]
    pub expires_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LifecycleRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exposures: Option<LifecycleRequestExposuresList>,
}

pub type LifecycleRequestExposuresList = Vec<LifecycleExposureRequest>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LifecycleResponse {
    #[serde(rename = "assetId")]
    pub asset_id: String,
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "previousState")]
    pub previous_state: String,
    pub state: String,
    #[serde(rename = "lockVersion")]
    pub lock_version: i64,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MetadataEntry {
    pub key: String,
    pub kind: String,
    pub value: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OperationContextPayload {
    pub initiator: String,
    pub action: String,
    #[serde(rename = "resourceUrn")]
    pub resource_urn: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OperationResponse {
    #[serde(rename = "operationId")]
    pub operation_id: String,
    pub status: nimbus_sdk_core::lro::OperationStatus,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "expiresAt")]
    pub expires_at: Option<String>,
    pub context: OperationContextPayload,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Value>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "progressStream"
    )]
    pub progress_stream: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PendingAssetVersion {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "assetId")]
    pub asset_id: String,
    #[serde(rename = "versionId")]
    pub version_id: String,
    pub ordinal: i32,
    #[serde(rename = "storageKey")]
    pub storage_key: String,
    #[serde(rename = "checksumAlgorithm")]
    pub checksum_algorithm: String,
    #[serde(rename = "declaredChecksum")]
    pub declared_checksum: String,
    #[serde(rename = "contentLength")]
    pub content_length: i64,
    pub status: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PendingUploadVersion {
    #[serde(rename = "versionId")]
    pub version_id: String,
    #[serde(rename = "storageKey")]
    pub storage_key: String,
    pub status: String,
    #[serde(rename = "checksumAlgorithm")]
    pub checksum_algorithm: String,
    pub checksum: String,
    #[serde(rename = "contentLength")]
    pub content_length: i64,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PipelineListResponse {
    pub items: PipelineListResponseItemsList,
}

pub type PipelineListResponseItemsList = Vec<PipelineRecordResponse>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PipelineRecordResponse {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "pipelineId")]
    pub pipeline_id: String,
    #[serde(rename = "pipelineRef")]
    pub pipeline_ref: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "pipelinePrefix"
    )]
    pub pipeline_prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cdnHost")]
    pub cdn_host: Option<String>,
    pub enabled: bool,
    pub renditions: PipelineRecordResponseRenditionsList,
    pub rules: PipelineRecordResponseRulesList,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "lockVersion")]
    pub lock_version: i64,
}

pub type PipelineRecordResponseRenditionsList = Vec<PipelineRenditionResponse>;

pub type PipelineRecordResponseRulesList = Vec<PipelineRuleResponse>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PipelineRenditionRequest {
    pub name: String,
    pub pipeline: String,
    #[serde(rename = "contentType")]
    pub content_type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PipelineRenditionResponse {
    pub name: String,
    pub pipeline: String,
    #[serde(rename = "contentType")]
    pub content_type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PipelineRerunRequest {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "pipelineRef"
    )]
    pub pipeline_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PipelineRerunResponse {
    pub operations: PipelineRerunResponseOperationsList,
    #[serde(rename = "matchedAssets")]
    pub matched_assets: i64,
    #[serde(rename = "queuedJobs")]
    pub queued_jobs: i64,
    #[serde(rename = "skippedAssets")]
    pub skipped_assets: i64,
    #[serde(rename = "failedAssets")]
    pub failed_assets: i64,
}

pub type PipelineRerunResponseOperationsList = Vec<OperationResponse>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PipelineRuleRequest {
    pub field: String,
    pub operator: String,
    pub value: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PipelineRuleResponse {
    pub field: String,
    pub operator: String,
    pub value: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProcessorCallbackPayload {
    pub status: String,
    #[serde(rename = "jobId")]
    pub job_id: String,
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "pipelineRef")]
    pub pipeline_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "storageKey"
    )]
    pub storage_key: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "checksumSha256"
    )]
    pub checksum_sha256: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sizeBytes")]
    pub size_bytes: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "contentType"
    )]
    pub content_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "assetId")]
    pub asset_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "versionId")]
    pub version_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<Value>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "pipelineFingerprint"
    )]
    pub pipeline_fingerprint: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "cacheHints"
    )]
    pub cache_hints: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "computeMillis"
    )]
    pub compute_millis: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegisteredAsset {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "bucketName")]
    pub bucket_name: String,
    #[serde(rename = "assetId")]
    pub asset_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub metadata: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SmartAlbumListResponse {
    pub items: SmartAlbumListResponseItemsList,
}

pub type SmartAlbumListResponseItemsList = Vec<SmartAlbumResponse>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SmartAlbumRecordResponse {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "albumId")]
    pub album_id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub definition: Value,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "lockVersion")]
    pub lock_version: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SmartAlbumResponse {
    pub album: SmartAlbumRecordResponse,
    pub rules: SmartAlbumResponseRulesList,
}

pub type SmartAlbumResponseRulesList = Vec<SmartAlbumRuleResponse>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SmartAlbumRuleRequest {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ruleId")]
    pub rule_id: Option<String>,
    pub field: String,
    pub operator: String,
    pub value: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SmartAlbumRuleResponse {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "albumId")]
    pub album_id: String,
    #[serde(rename = "ruleId")]
    pub rule_id: String,
    pub field: String,
    pub operator: String,
    pub value: Value,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateCollectionRequest {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "displayName"
    )]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "expectedLockVersion"
    )]
    pub expected_lock_version: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdatePipelineRequest {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "displayName"
    )]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "pipelinePrefix"
    )]
    pub pipeline_prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cdnHost")]
    pub cdn_host: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renditions: Option<UpdatePipelineRequestRenditionsList>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<UpdatePipelineRequestRulesList>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "expectedLockVersion"
    )]
    pub expected_lock_version: Option<i64>,
}

pub type UpdatePipelineRequestRenditionsList = Vec<PipelineRenditionRequest>;

pub type UpdatePipelineRequestRulesList = Vec<PipelineRuleRequest>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateSmartAlbumRequest {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "displayName"
    )]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<Value>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "expectedLockVersion"
    )]
    pub expected_lock_version: Option<i64>,
    pub rules: UpdateSmartAlbumRequestRulesList,
}

pub type UpdateSmartAlbumRequestRulesList = Vec<SmartAlbumRuleRequest>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UploadTicket {
    #[serde(rename = "uploadUrl")]
    pub upload_url: String,
    #[serde(rename = "uploadToken")]
    pub upload_token: String,
    #[serde(rename = "expiresAt")]
    pub expires_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UploadedAssetVersion {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "assetId")]
    pub asset_id: String,
    #[serde(rename = "versionId")]
    pub version_id: String,
    pub ordinal: i32,
    #[serde(rename = "storageKey")]
    pub storage_key: String,
    #[serde(rename = "checksumAlgorithm")]
    pub checksum_algorithm: String,
    pub checksum: String,
    #[serde(rename = "sizeBytes")]
    pub size_bytes: i64,
    pub status: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UploadedVersionPayload {
    #[serde(rename = "versionId")]
    pub version_id: String,
    pub ordinal: i32,
    #[serde(rename = "storageKey")]
    pub storage_key: String,
    #[serde(rename = "checksumAlgorithm")]
    pub checksum_algorithm: String,
    pub checksum: String,
    #[serde(rename = "sizeBytes")]
    pub size_bytes: i64,
    pub status: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}
