//! Generated types – do not edit by hand.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CancelKeyDeletionRequest {
    #[serde(rename = "requestId")]
    pub request_id: Value,
    #[serde(rename = "idempotencyKey")]
    pub idempotency_key: Value,
    #[serde(rename = "tenantId")]
    pub tenant_id: Value,
    #[serde(rename = "principalId")]
    pub principal_id: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateKeyRequest {
    #[serde(rename = "requestId")]
    pub request_id: Value,
    #[serde(rename = "idempotencyKey")]
    pub idempotency_key: Value,
    #[serde(rename = "tenantId")]
    pub tenant_id: Value,
    #[serde(rename = "principalId")]
    pub principal_id: Value,
    pub region: Value,
    #[serde(rename = "keySpec")]
    pub key_spec: KeySpec,
    #[serde(rename = "keyUsage")]
    pub key_usage: KeyUsage,
    pub algorithm: EncryptionAlgorithm,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<KeyTagsMap>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecryptDataKeyRequest {
    #[serde(rename = "requestId")]
    pub request_id: Value,
    #[serde(rename = "idempotencyKey")]
    pub idempotency_key: Value,
    #[serde(rename = "tenantId")]
    pub tenant_id: Value,
    #[serde(rename = "principalId")]
    pub principal_id: Value,
    #[serde(rename = "keyId")]
    pub key_id: Value,
    #[serde(rename = "ciphertextFormat")]
    pub ciphertext_format: Value,
    #[serde(rename = "encryptedDataKey")]
    pub encrypted_data_key: Vec<u8>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "encryptionContext"
    )]
    pub encryption_context: Option<EncryptionContextMap>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecryptDataKeyResponse {
    #[serde(rename = "requestId")]
    pub request_id: Value,
    #[serde(rename = "keyId")]
    pub key_id: Value,
    #[serde(rename = "plaintextDataKey")]
    pub plaintext_data_key: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecryptRequest {
    #[serde(rename = "requestId")]
    pub request_id: Value,
    #[serde(rename = "idempotencyKey")]
    pub idempotency_key: Value,
    #[serde(rename = "tenantId")]
    pub tenant_id: Value,
    #[serde(rename = "principalId")]
    pub principal_id: Value,
    #[serde(rename = "keyId")]
    pub key_id: Value,
    pub algorithm: EncryptionAlgorithm,
    #[serde(rename = "ciphertextFormat")]
    pub ciphertext_format: Value,
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "encryptionContext"
    )]
    pub encryption_context: Option<EncryptionContextMap>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecryptResponse {
    #[serde(rename = "requestId")]
    pub request_id: Value,
    #[serde(rename = "keyId")]
    pub key_id: Value,
    pub plaintext: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DisableKeyRequest {
    #[serde(rename = "requestId")]
    pub request_id: Value,
    #[serde(rename = "idempotencyKey")]
    pub idempotency_key: Value,
    #[serde(rename = "tenantId")]
    pub tenant_id: Value,
    #[serde(rename = "principalId")]
    pub principal_id: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnableKeyRequest {
    #[serde(rename = "requestId")]
    pub request_id: Value,
    #[serde(rename = "idempotencyKey")]
    pub idempotency_key: Value,
    #[serde(rename = "tenantId")]
    pub tenant_id: Value,
    #[serde(rename = "principalId")]
    pub principal_id: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EncryptRequest {
    #[serde(rename = "requestId")]
    pub request_id: Value,
    #[serde(rename = "idempotencyKey")]
    pub idempotency_key: Value,
    #[serde(rename = "tenantId")]
    pub tenant_id: Value,
    #[serde(rename = "principalId")]
    pub principal_id: Value,
    #[serde(rename = "keyId")]
    pub key_id: Value,
    pub algorithm: EncryptionAlgorithm,
    pub plaintext: Vec<u8>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "encryptionContext"
    )]
    pub encryption_context: Option<EncryptionContextMap>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EncryptResponse {
    #[serde(rename = "requestId")]
    pub request_id: Value,
    #[serde(rename = "keyId")]
    pub key_id: Value,
    pub algorithm: EncryptionAlgorithm,
    #[serde(rename = "ciphertextFormat")]
    pub ciphertext_format: Value,
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "encryptionContextHash"
    )]
    pub encryption_context_hash: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum EncryptionAlgorithm {
    #[serde(rename = "AES_128_GCM")]
    AES128GCM,
    #[serde(rename = "AES_256_GCM")]
    AES256GCM,
    #[serde(rename = "CHACHA20_POLY1305")]
    CHACHA20POLY1305,
}

pub type EncryptionContextMap = BTreeMap<String, Value>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GenerateDataKeyRequest {
    #[serde(rename = "requestId")]
    pub request_id: Value,
    #[serde(rename = "idempotencyKey")]
    pub idempotency_key: Value,
    #[serde(rename = "tenantId")]
    pub tenant_id: Value,
    #[serde(rename = "principalId")]
    pub principal_id: Value,
    #[serde(rename = "keyId")]
    pub key_id: Value,
    pub algorithm: EncryptionAlgorithm,
    #[serde(rename = "keyBytes")]
    pub key_bytes: i32,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "encryptionContext"
    )]
    pub encryption_context: Option<EncryptionContextMap>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GenerateDataKeyResponse {
    #[serde(rename = "requestId")]
    pub request_id: Value,
    #[serde(rename = "keyId")]
    pub key_id: Value,
    #[serde(rename = "ciphertextFormat")]
    pub ciphertext_format: Value,
    #[serde(rename = "plaintextDataKey")]
    pub plaintext_data_key: Vec<u8>,
    #[serde(rename = "encryptedDataKey")]
    pub encrypted_data_key: Vec<u8>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "encryptionContextHash"
    )]
    pub encryption_context_hash: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyMetadataResponse {
    #[serde(rename = "requestId")]
    pub request_id: Value,
    #[serde(rename = "keyId")]
    pub key_id: Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<Value>,
    #[serde(rename = "tenantId")]
    pub tenant_id: Value,
    pub region: Value,
    pub state: KeyState,
    #[serde(rename = "keySpec")]
    pub key_spec: KeySpec,
    #[serde(rename = "keyUsage")]
    pub key_usage: KeyUsage,
    pub algorithm: EncryptionAlgorithm,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "pendingDeletionAt"
    )]
    pub pending_deletion_at: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<KeyTagsMap>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum KeySpec {
    SymmetricDefault,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum KeyState {
    Pending,
    Active,
    Disabled,
    ScheduledDeletion,
    Deleted,
}

pub type KeyTagsMap = BTreeMap<String, String>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum KeyUsage {
    EncryptDecrypt,
    DataKeyWrapping,
    SignVerify,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PurgeKeyRequest {
    #[serde(rename = "requestId")]
    pub request_id: Value,
    #[serde(rename = "idempotencyKey")]
    pub idempotency_key: Value,
    #[serde(rename = "tenantId")]
    pub tenant_id: Value,
    #[serde(rename = "principalId")]
    pub principal_id: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RotateKeyRequest {
    #[serde(rename = "requestId")]
    pub request_id: Value,
    #[serde(rename = "idempotencyKey")]
    pub idempotency_key: Value,
    #[serde(rename = "tenantId")]
    pub tenant_id: Value,
    #[serde(rename = "principalId")]
    pub principal_id: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScheduleKeyDeletionRequest {
    #[serde(rename = "requestId")]
    pub request_id: Value,
    #[serde(rename = "idempotencyKey")]
    pub idempotency_key: Value,
    #[serde(rename = "tenantId")]
    pub tenant_id: Value,
    #[serde(rename = "principalId")]
    pub principal_id: Value,
    #[serde(rename = "pendingWindowDays")]
    pub pending_window_days: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignRequest {
    #[serde(rename = "requestId")]
    pub request_id: Value,
    #[serde(rename = "idempotencyKey")]
    pub idempotency_key: Value,
    #[serde(rename = "tenantId")]
    pub tenant_id: Value,
    #[serde(rename = "principalId")]
    pub principal_id: Value,
    #[serde(rename = "keyId")]
    pub key_id: Value,
    #[serde(rename = "signingAlgorithm")]
    pub signing_algorithm: SigningAlgorithm,
    pub message: Vec<u8>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "signingContext"
    )]
    pub signing_context: Option<EncryptionContextMap>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignResponse {
    #[serde(rename = "requestId")]
    pub request_id: Value,
    #[serde(rename = "keyId")]
    pub key_id: Value,
    #[serde(rename = "signingAlgorithm")]
    pub signing_algorithm: SigningAlgorithm,
    #[serde(rename = "signatureFormat")]
    pub signature_format: String,
    pub signature: Vec<u8>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "signingContextHash"
    )]
    pub signing_context_hash: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SigningAlgorithm {
    #[serde(rename = "HMAC_SHA256")]
    HMACSHA256,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VerifyRequest {
    #[serde(rename = "requestId")]
    pub request_id: Value,
    #[serde(rename = "idempotencyKey")]
    pub idempotency_key: Value,
    #[serde(rename = "tenantId")]
    pub tenant_id: Value,
    #[serde(rename = "principalId")]
    pub principal_id: Value,
    #[serde(rename = "keyId")]
    pub key_id: Value,
    #[serde(rename = "signingAlgorithm")]
    pub signing_algorithm: SigningAlgorithm,
    pub message: Vec<u8>,
    pub signature: Vec<u8>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "signingContext"
    )]
    pub signing_context: Option<EncryptionContextMap>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VerifyResponse {
    #[serde(rename = "requestId")]
    pub request_id: Value,
    #[serde(rename = "keyId")]
    pub key_id: Value,
    #[serde(rename = "signingAlgorithm")]
    pub signing_algorithm: SigningAlgorithm,
    pub valid: bool,
}
