//! Generated client – do not edit by hand.
#![allow(clippy::all)]

use crate::types::*;
use nimbus_sdk_core::{
    client::{NimbusClient, OperationSpec, SdkConfig},
    error::SdkError,
    lro::LroWaiter,
    SdkHttpMethod,
};
use serde_json::Value;

#[derive(Clone)]
pub struct DamManagementClient {
    inner: NimbusClient,
}

impl DamManagementClient {
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

    pub async fn archive_asset(
        &self,
        params: ArchiveAssetPathParams<'_>,
        body: &LifecycleRequest,
    ) -> Result<LifecycleResponse, SdkError> {
        let path_params = vec![("asset_id", params.asset_id.to_string())];
        let result = self
            .inner
            .invoke(&ARCHIVE_ASSET_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<LifecycleResponse>(result.body)
    }

    pub async fn begin_asset_ingestion(
        &self,
        body: &IngestionRequest,
    ) -> Result<IngestionResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&BEGIN_ASSET_INGESTION_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<IngestionResponse>(result.body)
    }

    pub async fn complete_asset_ingestion(
        &self,
        body: &CompleteIngestionRequest,
    ) -> Result<OperationResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(
                &COMPLETE_ASSET_INGESTION_SPEC,
                &path_params,
                Some(body),
                None,
            )
            .await?;
        self.inner.deserialize::<OperationResponse>(result.body)
    }

    pub async fn create_asset_download(
        &self,
        params: CreateAssetDownloadPathParams<'_>,
    ) -> Result<AssetDownloadResponse, SdkError> {
        let path_params = vec![("asset_id", params.asset_id.to_string())];
        let result = self
            .inner
            .invoke(&CREATE_ASSET_DOWNLOAD_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<AssetDownloadResponse>(result.body)
    }

    pub async fn create_asset_prefix(
        &self,
        body: &CreateAssetPrefixRequest,
    ) -> Result<AssetPrefixResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&CREATE_ASSET_PREFIX_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<AssetPrefixResponse>(result.body)
    }

    pub async fn create_collection(
        &self,
        body: &CreateCollectionRequest,
    ) -> Result<CollectionResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&CREATE_COLLECTION_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<CollectionResponse>(result.body)
    }

    pub async fn create_pipeline(
        &self,
        body: &CreatePipelineRequest,
    ) -> Result<PipelineRecordResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&CREATE_PIPELINE_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner
            .deserialize::<PipelineRecordResponse>(result.body)
    }

    pub async fn create_smart_album(
        &self,
        body: &CreateSmartAlbumRequest,
    ) -> Result<SmartAlbumResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&CREATE_SMART_ALBUM_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<SmartAlbumResponse>(result.body)
    }

    pub async fn delete_asset_prefix(
        &self,
        params: DeleteAssetPrefixPathParams<'_>,
    ) -> Result<Value, SdkError> {
        let path_params = vec![("prefix_id", params.prefix_id.to_string())];
        let result = self
            .inner
            .invoke(&DELETE_ASSET_PREFIX_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<Value>(result.body)
    }

    pub async fn delete_collection(
        &self,
        params: DeleteCollectionPathParams<'_>,
    ) -> Result<Value, SdkError> {
        let path_params = vec![("collection_id", params.collection_id.to_string())];
        let result = self
            .inner
            .invoke(&DELETE_COLLECTION_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<Value>(result.body)
    }

    pub async fn delete_pipeline(
        &self,
        params: DeletePipelinePathParams<'_>,
    ) -> Result<Value, SdkError> {
        let path_params = vec![("pipeline_id", params.pipeline_id.to_string())];
        let result = self
            .inner
            .invoke(&DELETE_PIPELINE_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<Value>(result.body)
    }

    pub async fn delete_smart_album(
        &self,
        params: DeleteSmartAlbumPathParams<'_>,
    ) -> Result<Value, SdkError> {
        let path_params = vec![("album_id", params.album_id.to_string())];
        let result = self
            .inner
            .invoke(&DELETE_SMART_ALBUM_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<Value>(result.body)
    }

    pub async fn get_asset(
        &self,
        params: GetAssetPathParams<'_>,
    ) -> Result<AssetDetailResponse, SdkError> {
        let path_params = vec![("asset_id", params.asset_id.to_string())];
        let result = self
            .inner
            .invoke(&GET_ASSET_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<AssetDetailResponse>(result.body)
    }

    pub async fn get_operation(
        &self,
        params: GetOperationPathParams<'_>,
    ) -> Result<OperationResponse, SdkError> {
        let path_params = vec![("operation_id", params.operation_id.to_string())];
        let result = self
            .inner
            .invoke(&GET_OPERATION_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<OperationResponse>(result.body)
    }

    pub async fn list_asset_prefixes(&self) -> Result<AssetPrefixListResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&LIST_ASSET_PREFIXES_SPEC, &path_params, None, None)
            .await?;
        self.inner
            .deserialize::<AssetPrefixListResponse>(result.body)
    }

    pub async fn list_collection_memberships(
        &self,
        params: ListCollectionMembershipsPathParams<'_>,
    ) -> Result<CollectionMembershipListResponse, SdkError> {
        let path_params = vec![("collection_id", params.collection_id.to_string())];
        let result = self
            .inner
            .invoke(&LIST_COLLECTION_MEMBERSHIPS_SPEC, &path_params, None, None)
            .await?;
        self.inner
            .deserialize::<CollectionMembershipListResponse>(result.body)
    }

    pub async fn list_collections(&self) -> Result<CollectionListResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&LIST_COLLECTIONS_SPEC, &path_params, None, None)
            .await?;
        self.inner
            .deserialize::<CollectionListResponse>(result.body)
    }

    pub async fn list_pipelines(&self) -> Result<PipelineListResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&LIST_PIPELINES_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<PipelineListResponse>(result.body)
    }

    pub async fn list_smart_albums(&self) -> Result<SmartAlbumListResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&LIST_SMART_ALBUMS_SPEC, &path_params, None, None)
            .await?;
        self.inner
            .deserialize::<SmartAlbumListResponse>(result.body)
    }

    pub async fn operation_callback(
        &self,
        params: OperationCallbackPathParams<'_>,
        body: &ProcessorCallbackPayload,
    ) -> Result<Value, SdkError> {
        let path_params = vec![("token", params.token.to_string())];
        let result = self
            .inner
            .invoke(&OPERATION_CALLBACK_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<Value>(result.body)
    }

    pub async fn publish_asset(
        &self,
        params: PublishAssetPathParams<'_>,
        body: &LifecycleRequest,
    ) -> Result<LifecycleResponse, SdkError> {
        let path_params = vec![("asset_id", params.asset_id.to_string())];
        let result = self
            .inner
            .invoke(&PUBLISH_ASSET_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<LifecycleResponse>(result.body)
    }

    pub async fn put_collection_memberships(
        &self,
        params: PutCollectionMembershipsPathParams<'_>,
        body: &CollectionMembershipChangeRequest,
    ) -> Result<CollectionMembershipListResponse, SdkError> {
        let path_params = vec![("collection_id", params.collection_id.to_string())];
        let result = self
            .inner
            .invoke(
                &PUT_COLLECTION_MEMBERSHIPS_SPEC,
                &path_params,
                Some(body),
                None,
            )
            .await?;
        self.inner
            .deserialize::<CollectionMembershipListResponse>(result.body)
    }

    pub async fn record_index_snapshot(
        &self,
        body: &IndexSnapshotRequest,
    ) -> Result<IndexSnapshotResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&RECORD_INDEX_SNAPSHOT_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<IndexSnapshotResponse>(result.body)
    }

    pub async fn remove_collection_membership(
        &self,
        params: RemoveCollectionMembershipPathParams<'_>,
    ) -> Result<Value, SdkError> {
        let path_params = vec![
            ("collection_id", params.collection_id.to_string()),
            ("asset_id", params.asset_id.to_string()),
        ];
        let result = self
            .inner
            .invoke(&REMOVE_COLLECTION_MEMBERSHIP_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<Value>(result.body)
    }

    pub async fn restore_asset(
        &self,
        params: RestoreAssetPathParams<'_>,
        body: &LifecycleRequest,
    ) -> Result<LifecycleResponse, SdkError> {
        let path_params = vec![("asset_id", params.asset_id.to_string())];
        let result = self
            .inner
            .invoke(&RESTORE_ASSET_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<LifecycleResponse>(result.body)
    }

    pub async fn retry_operation(
        &self,
        params: RetryOperationPathParams<'_>,
    ) -> Result<OperationResponse, SdkError> {
        let path_params = vec![("operation_id", params.operation_id.to_string())];
        let result = self
            .inner
            .invoke(&RETRY_OPERATION_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<OperationResponse>(result.body)
    }

    pub async fn search_assets(
        &self,
        body: &AssetSearchRequest,
    ) -> Result<AssetSearchResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&SEARCH_ASSETS_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<AssetSearchResponse>(result.body)
    }

    pub async fn search_assets_with_body(
        &self,
        body: &AssetSearchRequest,
    ) -> Result<AssetSearchResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(
                &SEARCH_ASSETS_WITH_BODY_SPEC,
                &path_params,
                Some(body),
                None,
            )
            .await?;
        self.inner.deserialize::<AssetSearchResponse>(result.body)
    }

    pub async fn update_collection(
        &self,
        params: UpdateCollectionPathParams<'_>,
        body: &UpdateCollectionRequest,
    ) -> Result<CollectionResponse, SdkError> {
        let path_params = vec![("collection_id", params.collection_id.to_string())];
        let result = self
            .inner
            .invoke(&UPDATE_COLLECTION_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<CollectionResponse>(result.body)
    }

    pub async fn update_pipeline(
        &self,
        params: UpdatePipelinePathParams<'_>,
        body: &UpdatePipelineRequest,
    ) -> Result<PipelineRecordResponse, SdkError> {
        let path_params = vec![("pipeline_id", params.pipeline_id.to_string())];
        let result = self
            .inner
            .invoke(&UPDATE_PIPELINE_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner
            .deserialize::<PipelineRecordResponse>(result.body)
    }

    pub async fn update_smart_album(
        &self,
        params: UpdateSmartAlbumPathParams<'_>,
        body: &UpdateSmartAlbumRequest,
    ) -> Result<SmartAlbumResponse, SdkError> {
        let path_params = vec![("album_id", params.album_id.to_string())];
        let result = self
            .inner
            .invoke(&UPDATE_SMART_ALBUM_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<SmartAlbumResponse>(result.body)
    }

    pub fn waiter(&self) -> LroWaiter {
        self.inner.waiter()
    }
}

#[derive(Clone, Debug)]
pub struct ArchiveAssetPathParams<'a> {
    pub asset_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct CreateAssetDownloadPathParams<'a> {
    pub asset_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct DeleteAssetPrefixPathParams<'a> {
    pub prefix_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct DeleteCollectionPathParams<'a> {
    pub collection_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct DeletePipelinePathParams<'a> {
    pub pipeline_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct DeleteSmartAlbumPathParams<'a> {
    pub album_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct GetAssetPathParams<'a> {
    pub asset_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct GetOperationPathParams<'a> {
    pub operation_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct ListCollectionMembershipsPathParams<'a> {
    pub collection_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct OperationCallbackPathParams<'a> {
    pub token: &'a str,
}

#[derive(Clone, Debug)]
pub struct PublishAssetPathParams<'a> {
    pub asset_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct PutCollectionMembershipsPathParams<'a> {
    pub collection_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct RemoveCollectionMembershipPathParams<'a> {
    pub collection_id: &'a str,
    pub asset_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct RestoreAssetPathParams<'a> {
    pub asset_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct RetryOperationPathParams<'a> {
    pub operation_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct UpdateCollectionPathParams<'a> {
    pub collection_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct UpdatePipelinePathParams<'a> {
    pub pipeline_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct UpdateSmartAlbumPathParams<'a> {
    pub album_id: &'a str,
}

const ARCHIVE_ASSET_SPEC: OperationSpec = OperationSpec {
    name: "ArchiveAsset",
    method: SdkHttpMethod::Post,
    uri: "/assets/{asset_id}/archive",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const BEGIN_ASSET_INGESTION_SPEC: OperationSpec = OperationSpec {
    name: "BeginAssetIngestion",
    method: SdkHttpMethod::Post,
    uri: "/assets/ingest",
    success_code: 202,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: true,
};

const COMPLETE_ASSET_INGESTION_SPEC: OperationSpec = OperationSpec {
    name: "CompleteAssetIngestion",
    method: SdkHttpMethod::Post,
    uri: "/assets/ingest/complete",
    success_code: 202,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: true,
};

const CREATE_ASSET_DOWNLOAD_SPEC: OperationSpec = OperationSpec {
    name: "CreateAssetDownload",
    method: SdkHttpMethod::Post,
    uri: "/assets/{asset_id}/download",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const CREATE_ASSET_PREFIX_SPEC: OperationSpec = OperationSpec {
    name: "CreateAssetPrefix",
    method: SdkHttpMethod::Post,
    uri: "/assets/prefixes",
    success_code: 201,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const CREATE_COLLECTION_SPEC: OperationSpec = OperationSpec {
    name: "CreateCollection",
    method: SdkHttpMethod::Post,
    uri: "/collections",
    success_code: 201,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const CREATE_PIPELINE_SPEC: OperationSpec = OperationSpec {
    name: "CreatePipeline",
    method: SdkHttpMethod::Post,
    uri: "/pipelines",
    success_code: 201,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const CREATE_SMART_ALBUM_SPEC: OperationSpec = OperationSpec {
    name: "CreateSmartAlbum",
    method: SdkHttpMethod::Post,
    uri: "/albums",
    success_code: 201,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const DELETE_ASSET_PREFIX_SPEC: OperationSpec = OperationSpec {
    name: "DeleteAssetPrefix",
    method: SdkHttpMethod::Delete,
    uri: "/assets/prefixes/{prefix_id}",
    success_code: 204,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const DELETE_COLLECTION_SPEC: OperationSpec = OperationSpec {
    name: "DeleteCollection",
    method: SdkHttpMethod::Delete,
    uri: "/collections/{collection_id}",
    success_code: 204,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const DELETE_PIPELINE_SPEC: OperationSpec = OperationSpec {
    name: "DeletePipeline",
    method: SdkHttpMethod::Delete,
    uri: "/pipelines/{pipeline_id}",
    success_code: 204,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const DELETE_SMART_ALBUM_SPEC: OperationSpec = OperationSpec {
    name: "DeleteSmartAlbum",
    method: SdkHttpMethod::Delete,
    uri: "/albums/{album_id}",
    success_code: 204,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const GET_ASSET_SPEC: OperationSpec = OperationSpec {
    name: "GetAsset",
    method: SdkHttpMethod::Get,
    uri: "/assets/{asset_id}",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const GET_OPERATION_SPEC: OperationSpec = OperationSpec {
    name: "GetOperation",
    method: SdkHttpMethod::Get,
    uri: "/operations/{operation_id}",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const LIST_ASSET_PREFIXES_SPEC: OperationSpec = OperationSpec {
    name: "ListAssetPrefixes",
    method: SdkHttpMethod::Get,
    uri: "/assets/prefixes",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const LIST_COLLECTION_MEMBERSHIPS_SPEC: OperationSpec = OperationSpec {
    name: "ListCollectionMemberships",
    method: SdkHttpMethod::Get,
    uri: "/collections/{collection_id}/memberships",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const LIST_COLLECTIONS_SPEC: OperationSpec = OperationSpec {
    name: "ListCollections",
    method: SdkHttpMethod::Get,
    uri: "/collections",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const LIST_PIPELINES_SPEC: OperationSpec = OperationSpec {
    name: "ListPipelines",
    method: SdkHttpMethod::Get,
    uri: "/pipelines",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const LIST_SMART_ALBUMS_SPEC: OperationSpec = OperationSpec {
    name: "ListSmartAlbums",
    method: SdkHttpMethod::Get,
    uri: "/albums",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const OPERATION_CALLBACK_SPEC: OperationSpec = OperationSpec {
    name: "OperationCallback",
    method: SdkHttpMethod::Post,
    uri: "/operations/callbacks/{token}",
    success_code: 204,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const PUBLISH_ASSET_SPEC: OperationSpec = OperationSpec {
    name: "PublishAsset",
    method: SdkHttpMethod::Post,
    uri: "/assets/{asset_id}/publish",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const PUT_COLLECTION_MEMBERSHIPS_SPEC: OperationSpec = OperationSpec {
    name: "PutCollectionMemberships",
    method: SdkHttpMethod::Post,
    uri: "/collections/{collection_id}/memberships",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const RECORD_INDEX_SNAPSHOT_SPEC: OperationSpec = OperationSpec {
    name: "RecordIndexSnapshot",
    method: SdkHttpMethod::Post,
    uri: "/index/snapshots",
    success_code: 201,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const REMOVE_COLLECTION_MEMBERSHIP_SPEC: OperationSpec = OperationSpec {
    name: "RemoveCollectionMembership",
    method: SdkHttpMethod::Delete,
    uri: "/collections/{collection_id}/memberships/{asset_id}",
    success_code: 204,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const RESTORE_ASSET_SPEC: OperationSpec = OperationSpec {
    name: "RestoreAsset",
    method: SdkHttpMethod::Post,
    uri: "/assets/{asset_id}/restore",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const RETRY_OPERATION_SPEC: OperationSpec = OperationSpec {
    name: "RetryOperation",
    method: SdkHttpMethod::Post,
    uri: "/operations/{operation_id}/retry",
    success_code: 202,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: true,
};

const SEARCH_ASSETS_SPEC: OperationSpec = OperationSpec {
    name: "SearchAssets",
    method: SdkHttpMethod::Get,
    uri: "/assets/search",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const SEARCH_ASSETS_WITH_BODY_SPEC: OperationSpec = OperationSpec {
    name: "SearchAssetsWithBody",
    method: SdkHttpMethod::Post,
    uri: "/assets/search",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const UPDATE_COLLECTION_SPEC: OperationSpec = OperationSpec {
    name: "UpdateCollection",
    method: SdkHttpMethod::Post,
    uri: "/collections/{collection_id}",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const UPDATE_PIPELINE_SPEC: OperationSpec = OperationSpec {
    name: "UpdatePipeline",
    method: SdkHttpMethod::Post,
    uri: "/pipelines/{pipeline_id}",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const UPDATE_SMART_ALBUM_SPEC: OperationSpec = OperationSpec {
    name: "UpdateSmartAlbum",
    method: SdkHttpMethod::Post,
    uri: "/albums/{album_id}",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};
