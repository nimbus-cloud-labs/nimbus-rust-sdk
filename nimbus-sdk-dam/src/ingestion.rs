use std::{convert::TryFrom, io};

use bytes::Bytes;
use nimbus_sdk_core::{transport::TransportError, SdkError};
use reqwest::Client;
use serde::Deserialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

use crate::{client::DamManagementClient, types::OperationResponse};

/// Parameters describing an ingestion request.
#[derive(Debug, Clone)]
pub struct IngestionParams {
    pub display_name: String,
    pub media_type: String,
    pub metadata: Value,
}

impl IngestionParams {
    pub fn with_metadata(
        display_name: impl Into<String>,
        media_type: impl Into<String>,
        metadata: Value,
    ) -> Self {
        Self {
            display_name: display_name.into(),
            media_type: media_type.into(),
            metadata,
        }
    }
}

/// Convenience wrapper that drives the multi-step ingestion flow using the DAM API and object storage layer.
/// The underlying `DamManagementClient` should be configured with an IAM-aware `AuthTokenProvider`.
#[derive(Clone)]
pub struct DamIngestionUploader {
    api: DamManagementClient,
    storage: Client,
}

impl DamIngestionUploader {
    pub fn new(api: DamManagementClient) -> Self {
        Self {
            api,
            storage: Client::new(),
        }
    }

    pub fn with_http_client(api: DamManagementClient, storage: Client) -> Self {
        Self { api, storage }
    }

    /// Uploads raw bytes using the DAM ingestion pipeline, returning the resulting operation metadata.
    pub async fn upload_bytes(
        &self,
        params: IngestionParams,
        content: Bytes,
    ) -> Result<OperationResponse, SdkError> {
        let metadata = if params.metadata.is_null() {
            json!({})
        } else {
            params.metadata
        };
        let size_bytes = i64::try_from(content.len())
            .map_err(|_| payload_error("payload size exceeds supported range"))?;
        let checksum = hex::encode(Sha256::digest(&content));
        let ingestion_request = json!({
            "display_name": params.display_name,
            "media_type": params.media_type,
            "content_length": size_bytes,
            "checksum_algorithm": "sha256",
            "checksum": checksum,
            "metadata": metadata,
        });

        let begin_value = self.api.begin_asset_ingestion(&ingestion_request).await?;
        let begin_response: BeginResponse = serde_json::from_value(begin_value.clone())?;
        self.put_object(
            &begin_response.upload.upload_url,
            &begin_response.upload.upload_token,
            &params.media_type,
            content,
        )
        .await?;

        let complete_request = json!({
            "upload_token": begin_response.upload.upload_token,
            "checksum_algorithm": "sha256",
            "checksum": checksum,
            "size_bytes": size_bytes,
        });

        self.api.complete_asset_ingestion(&complete_request).await
    }

    async fn put_object(
        &self,
        url: &str,
        token: &str,
        media_type: &str,
        body: Bytes,
    ) -> Result<(), SdkError> {
        let response = self
            .storage
            .put(url)
            .header("x-nimbus-upload-token", token)
            .header("content-type", media_type)
            .body(body)
            .send()
            .await
            .map_err(|err| SdkError::from(TransportError::Http(err)))?;

        if !response.status().is_success() {
            return Err(SdkError::UnexpectedStatus {
                operation: "UploadObject",
                status: response.status().as_u16(),
            });
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
struct BeginResponse {
    upload: UploadTicketPayload,
}

#[derive(Debug, Deserialize)]
struct UploadTicketPayload {
    upload_url: String,
    upload_token: String,
}

fn payload_error(message: &str) -> SdkError {
    SdkError::Json(serde_json::Error::io(io::Error::other(message.to_string())))
}
