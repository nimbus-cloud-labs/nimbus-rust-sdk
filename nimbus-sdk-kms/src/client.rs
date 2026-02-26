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
        let body_value = serde_json::to_value(body).map_err(SdkError::Json)?;
        let result = self
            .inner
            .invoke(
                &CANCEL_KEY_DELETION_SPEC,
                &path_params,
                Some(&body_value),
                None,
            )
            .await?;
        self.inner.deserialize::<KeyMetadataResponse>(result.body)
    }

    pub async fn create_key(
        &self,
        body: &CreateKeyRequest,
    ) -> Result<KeyMetadataResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let body_value = serde_json::to_value(body).map_err(SdkError::Json)?;
        let result = self
            .inner
            .invoke(&CREATE_KEY_SPEC, &path_params, Some(&body_value), None)
            .await?;
        self.inner.deserialize::<KeyMetadataResponse>(result.body)
    }

    pub async fn decrypt(&self, body: &DecryptRequest) -> Result<DecryptResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let body_value = serde_json::to_value(body).map_err(SdkError::Json)?;
        let result = self
            .inner
            .invoke(&DECRYPT_SPEC, &path_params, Some(&body_value), None)
            .await?;
        self.inner.deserialize::<DecryptResponse>(result.body)
    }

    pub async fn decrypt_data_key(
        &self,
        body: &DecryptDataKeyRequest,
    ) -> Result<DecryptDataKeyResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let body_value = serde_json::to_value(body).map_err(SdkError::Json)?;
        let result = self
            .inner
            .invoke(
                &DECRYPT_DATA_KEY_SPEC,
                &path_params,
                Some(&body_value),
                None,
            )
            .await?;
        self.inner
            .deserialize::<DecryptDataKeyResponse>(result.body)
    }

    pub async fn encrypt(&self, body: &EncryptRequest) -> Result<EncryptResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let body_value = serde_json::to_value(body).map_err(SdkError::Json)?;
        let result = self
            .inner
            .invoke(&ENCRYPT_SPEC, &path_params, Some(&body_value), None)
            .await?;
        self.inner.deserialize::<EncryptResponse>(result.body)
    }

    pub async fn generate_data_key(
        &self,
        body: &GenerateDataKeyRequest,
    ) -> Result<GenerateDataKeyResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let body_value = serde_json::to_value(body).map_err(SdkError::Json)?;
        let result = self
            .inner
            .invoke(
                &GENERATE_DATA_KEY_SPEC,
                &path_params,
                Some(&body_value),
                None,
            )
            .await?;
        self.inner
            .deserialize::<GenerateDataKeyResponse>(result.body)
    }

    pub async fn disable_key(
        &self,
        params: DisableKeyPathParams<'_>,
        body: &DisableKeyRequest,
    ) -> Result<KeyMetadataResponse, SdkError> {
        let path_params = vec![("key_id", params.key_id.to_string())];
        let body_value = serde_json::to_value(body).map_err(SdkError::Json)?;
        let result = self
            .inner
            .invoke(&DISABLE_KEY_SPEC, &path_params, Some(&body_value), None)
            .await?;
        self.inner.deserialize::<KeyMetadataResponse>(result.body)
    }

    pub async fn enable_key(
        &self,
        params: EnableKeyPathParams<'_>,
        body: &EnableKeyRequest,
    ) -> Result<KeyMetadataResponse, SdkError> {
        let path_params = vec![("key_id", params.key_id.to_string())];
        let body_value = serde_json::to_value(body).map_err(SdkError::Json)?;
        let result = self
            .inner
            .invoke(&ENABLE_KEY_SPEC, &path_params, Some(&body_value), None)
            .await?;
        self.inner.deserialize::<KeyMetadataResponse>(result.body)
    }

    pub async fn rotate_key(
        &self,
        params: RotateKeyPathParams<'_>,
        body: &RotateKeyRequest,
    ) -> Result<KeyMetadataResponse, SdkError> {
        let path_params = vec![("key_id", params.key_id.to_string())];
        let body_value = serde_json::to_value(body).map_err(SdkError::Json)?;
        let result = self
            .inner
            .invoke(&ROTATE_KEY_SPEC, &path_params, Some(&body_value), None)
            .await?;
        self.inner.deserialize::<KeyMetadataResponse>(result.body)
    }

    pub async fn purge_key(
        &self,
        params: PurgeKeyPathParams<'_>,
        body: &PurgeKeyRequest,
    ) -> Result<KeyMetadataResponse, SdkError> {
        let path_params = vec![("key_id", params.key_id.to_string())];
        let body_value = serde_json::to_value(body).map_err(SdkError::Json)?;
        let result = self
            .inner
            .invoke(&PURGE_KEY_SPEC, &path_params, Some(&body_value), None)
            .await?;
        self.inner.deserialize::<KeyMetadataResponse>(result.body)
    }

    pub async fn schedule_key_deletion(
        &self,
        params: ScheduleKeyDeletionPathParams<'_>,
        body: &ScheduleKeyDeletionRequest,
    ) -> Result<KeyMetadataResponse, SdkError> {
        let path_params = vec![("key_id", params.key_id.to_string())];
        let body_value = serde_json::to_value(body).map_err(SdkError::Json)?;
        let result = self
            .inner
            .invoke(
                &SCHEDULE_KEY_DELETION_SPEC,
                &path_params,
                Some(&body_value),
                None,
            )
            .await?;
        self.inner.deserialize::<KeyMetadataResponse>(result.body)
    }

    pub async fn sign(&self, body: &SignRequest) -> Result<SignResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let body_value = serde_json::to_value(body).map_err(SdkError::Json)?;
        let result = self
            .inner
            .invoke(&SIGN_SPEC, &path_params, Some(&body_value), None)
            .await?;
        self.inner.deserialize::<SignResponse>(result.body)
    }

    pub async fn verify(&self, body: &VerifyRequest) -> Result<VerifyResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let body_value = serde_json::to_value(body).map_err(SdkError::Json)?;
        let result = self
            .inner
            .invoke(&VERIFY_SPEC, &path_params, Some(&body_value), None)
            .await?;
        self.inner.deserialize::<VerifyResponse>(result.body)
    }
}

#[derive(Clone, Debug)]
pub struct CancelKeyDeletionPathParams<'a> {
    pub key_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct DisableKeyPathParams<'a> {
    pub key_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct EnableKeyPathParams<'a> {
    pub key_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct RotateKeyPathParams<'a> {
    pub key_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct PurgeKeyPathParams<'a> {
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

const DISABLE_KEY_SPEC: OperationSpec = OperationSpec {
    name: "DisableKey",
    method: SdkHttpMethod::Post,
    uri: "/keys/{key_id}/disable",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: true,
    pagination: None,
    lro: false,
};

const ENABLE_KEY_SPEC: OperationSpec = OperationSpec {
    name: "EnableKey",
    method: SdkHttpMethod::Post,
    uri: "/keys/{key_id}/enable",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: true,
    pagination: None,
    lro: false,
};

const ROTATE_KEY_SPEC: OperationSpec = OperationSpec {
    name: "RotateKey",
    method: SdkHttpMethod::Post,
    uri: "/keys/{key_id}/rotate",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: true,
    pagination: None,
    lro: false,
};

const PURGE_KEY_SPEC: OperationSpec = OperationSpec {
    name: "PurgeKey",
    method: SdkHttpMethod::Post,
    uri: "/keys/{key_id}/purge",
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

const SIGN_SPEC: OperationSpec = OperationSpec {
    name: "Sign",
    method: SdkHttpMethod::Post,
    uri: "/crypto/sign",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: true,
    pagination: None,
    lro: false,
};

const VERIFY_SPEC: OperationSpec = OperationSpec {
    name: "Verify",
    method: SdkHttpMethod::Post,
    uri: "/crypto/verify",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: true,
    pagination: None,
    lro: false,
};
