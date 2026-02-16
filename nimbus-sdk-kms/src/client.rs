//! Generated client – do not edit by hand.
#![allow(clippy::all)]

use crate::types::*;
use nimbus_sdk_core::{
    client::{NimbusClient, OperationSpec, SdkConfig},
    error::SdkError,
    SdkHttpMethod,
};

#[derive(Clone)]
pub struct KmsClient {
    inner: NimbusClient,
}

impl KmsClient {
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

    pub async fn cancel_key_deletion(
        &self,
        params: CancelKeyDeletionPathParams<'_>,
        body: &CancelKeyDeletionRequest,
    ) -> Result<KeyMetadataResponse, SdkError> {
        let path_params = vec![("key_id", params.key_id.to_string())];
        let result = self
            .inner
            .invoke(&CANCEL_KEY_DELETION_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<KeyMetadataResponse>(result.body)
    }

    pub async fn create_key(
        &self,
        body: &CreateKeyRequest,
    ) -> Result<KeyMetadataResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&CREATE_KEY_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<KeyMetadataResponse>(result.body)
    }

    pub async fn decrypt(&self, body: &DecryptRequest) -> Result<DecryptResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&DECRYPT_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<DecryptResponse>(result.body)
    }

    pub async fn decrypt_data_key(
        &self,
        body: &DecryptDataKeyRequest,
    ) -> Result<DecryptDataKeyResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&DECRYPT_DATA_KEY_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner
            .deserialize::<DecryptDataKeyResponse>(result.body)
    }

    pub async fn encrypt(&self, body: &EncryptRequest) -> Result<EncryptResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&ENCRYPT_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<EncryptResponse>(result.body)
    }

    pub async fn generate_data_key(
        &self,
        body: &GenerateDataKeyRequest,
    ) -> Result<GenerateDataKeyResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&GENERATE_DATA_KEY_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner
            .deserialize::<GenerateDataKeyResponse>(result.body)
    }

    pub async fn schedule_key_deletion(
        &self,
        params: ScheduleKeyDeletionPathParams<'_>,
        body: &ScheduleKeyDeletionRequest,
    ) -> Result<KeyMetadataResponse, SdkError> {
        let path_params = vec![("key_id", params.key_id.to_string())];
        let result = self
            .inner
            .invoke(&SCHEDULE_KEY_DELETION_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<KeyMetadataResponse>(result.body)
    }
}

#[derive(Clone, Debug)]
pub struct CancelKeyDeletionPathParams<'a> {
    pub key_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct ScheduleKeyDeletionPathParams<'a> {
    pub key_id: &'a str,
}

const CANCEL_KEY_DELETION_SPEC: OperationSpec = OperationSpec {
    name: "CancelKeyDeletion",
    method: SdkHttpMethod::Post,
    uri: "/keys/{key_id}/cancel-deletion",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: true,
    pagination: None,
    lro: false,
};

const CREATE_KEY_SPEC: OperationSpec = OperationSpec {
    name: "CreateKey",
    method: SdkHttpMethod::Post,
    uri: "/keys",
    success_code: 201,
    additional_success_responses: &[],
    idempotent: true,
    pagination: None,
    lro: false,
};

const DECRYPT_SPEC: OperationSpec = OperationSpec {
    name: "Decrypt",
    method: SdkHttpMethod::Post,
    uri: "/crypto/decrypt",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: true,
    pagination: None,
    lro: false,
};

const DECRYPT_DATA_KEY_SPEC: OperationSpec = OperationSpec {
    name: "DecryptDataKey",
    method: SdkHttpMethod::Post,
    uri: "/crypto/decrypt-data-key",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: true,
    pagination: None,
    lro: false,
};

const ENCRYPT_SPEC: OperationSpec = OperationSpec {
    name: "Encrypt",
    method: SdkHttpMethod::Post,
    uri: "/crypto/encrypt",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: true,
    pagination: None,
    lro: false,
};

const GENERATE_DATA_KEY_SPEC: OperationSpec = OperationSpec {
    name: "GenerateDataKey",
    method: SdkHttpMethod::Post,
    uri: "/crypto/generate-data-key",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: true,
    pagination: None,
    lro: false,
};

const SCHEDULE_KEY_DELETION_SPEC: OperationSpec = OperationSpec {
    name: "ScheduleKeyDeletion",
    method: SdkHttpMethod::Post,
    uri: "/keys/{key_id}/schedule-deletion",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: true,
    pagination: None,
    lro: false,
};
