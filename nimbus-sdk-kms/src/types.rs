//! Generated types – do not edit by hand.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CancelKeyDeletionRequest {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateKeyRequest {
    #[serde(rename = "keySpec")]
    pub key_spec: String,
    #[serde(rename = "keyUsage")]
    pub key_usage: String,
    pub algorithm: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<CreateKeyRequestTagsMap>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "trustPolicy"
    )]
    pub trust_policy: Option<Value>,
}

pub type CreateKeyRequestTagsMap = BTreeMap<String, String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecryptDataKeyRequest {
    #[serde(rename = "keyId")]
    pub key_id: String,
    #[serde(rename = "ciphertextFormat")]
    pub ciphertext_format: String,
    #[serde(rename = "encryptedDataKey")]
    pub encrypted_data_key: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "encryptionContext"
    )]
    pub encryption_context: Option<DecryptDataKeyRequestEncryptionContextMap>,
}

pub type DecryptDataKeyRequestEncryptionContextMap = BTreeMap<String, String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecryptDataKeyResponse {
    #[serde(rename = "keyId")]
    pub key_id: String,
    #[serde(rename = "plaintextDataKey")]
    pub plaintext_data_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecryptRequest {
    #[serde(rename = "keyId")]
    pub key_id: String,
    pub algorithm: String,
    #[serde(rename = "ciphertextFormat")]
    pub ciphertext_format: String,
    pub ciphertext: String,
    pub nonce: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "encryptionContext"
    )]
    pub encryption_context: Option<DecryptRequestEncryptionContextMap>,
}

pub type DecryptRequestEncryptionContextMap = BTreeMap<String, String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecryptResponse {
    #[serde(rename = "keyId")]
    pub key_id: String,
    pub plaintext: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DisableKeyRequest {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnableKeyRequest {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EncryptRequest {
    #[serde(rename = "keyId")]
    pub key_id: String,
    pub algorithm: String,
    pub plaintext: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "encryptionContext"
    )]
    pub encryption_context: Option<EncryptRequestEncryptionContextMap>,
}

pub type EncryptRequestEncryptionContextMap = BTreeMap<String, String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EncryptResponse {
    #[serde(rename = "keyId")]
    pub key_id: String,
    pub algorithm: String,
    #[serde(rename = "ciphertextFormat")]
    pub ciphertext_format: String,
    pub ciphertext: String,
    pub nonce: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "encryptionContextHash"
    )]
    pub encryption_context_hash: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GenerateDataKeyRequest {
    #[serde(rename = "keyId")]
    pub key_id: String,
    pub algorithm: String,
    #[serde(rename = "keyBytes")]
    pub key_bytes: i64,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "encryptionContext"
    )]
    pub encryption_context: Option<GenerateDataKeyRequestEncryptionContextMap>,
}

pub type GenerateDataKeyRequestEncryptionContextMap = BTreeMap<String, String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GenerateDataKeyResponse {
    #[serde(rename = "keyId")]
    pub key_id: String,
    #[serde(rename = "ciphertextFormat")]
    pub ciphertext_format: String,
    #[serde(rename = "plaintextDataKey")]
    pub plaintext_data_key: String,
    #[serde(rename = "encryptedDataKey")]
    pub encrypted_data_key: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "encryptionContextHash"
    )]
    pub encryption_context_hash: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyMetadataResponse {
    #[serde(rename = "keyId")]
    pub key_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    pub state: String,
    #[serde(rename = "keySpec")]
    pub key_spec: String,
    #[serde(rename = "keyUsage")]
    pub key_usage: String,
    pub algorithm: String,
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
    pub tags: Option<KeyMetadataResponseTagsMap>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "trustPolicy"
    )]
    pub trust_policy: Option<Value>,
}

pub type KeyMetadataResponseTagsMap = BTreeMap<String, String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PurgeKeyRequest {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PutTenantAlgorithmPolicyRequestPayload {
    #[serde(rename = "allowedAlgorithms")]
    pub allowed_algorithms: PutTenantAlgorithmPolicyRequestPayloadAllowedAlgorithmsList,
    #[serde(rename = "defaultAlgorithm")]
    pub default_algorithm: String,
}

pub type PutTenantAlgorithmPolicyRequestPayloadAllowedAlgorithmsList = Vec<String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PutTenantQuotaPolicyRequestPayload {
    #[serde(rename = "requestsPerSecond")]
    pub requests_per_second: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RotateKeyRequest {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScheduleKeyDeletionRequest {
    #[serde(rename = "pendingWindowDays")]
    pub pending_window_days: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignRequest {
    #[serde(rename = "keyId")]
    pub key_id: String,
    #[serde(rename = "signingAlgorithm")]
    pub signing_algorithm: String,
    pub message: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "signingContext"
    )]
    pub signing_context: Option<SignRequestSigningContextMap>,
}

pub type SignRequestSigningContextMap = BTreeMap<String, String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignResponse {
    #[serde(rename = "keyId")]
    pub key_id: String,
    #[serde(rename = "signingAlgorithm")]
    pub signing_algorithm: String,
    #[serde(rename = "signatureFormat")]
    pub signature_format: String,
    pub signature: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "signingContextHash"
    )]
    pub signing_context_hash: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VerifyRequest {
    #[serde(rename = "keyId")]
    pub key_id: String,
    #[serde(rename = "signingAlgorithm")]
    pub signing_algorithm: String,
    pub message: String,
    pub signature: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "signingContext"
    )]
    pub signing_context: Option<VerifyRequestSigningContextMap>,
}

pub type VerifyRequestSigningContextMap = BTreeMap<String, String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VerifyResponse {
    #[serde(rename = "keyId")]
    pub key_id: String,
    #[serde(rename = "signingAlgorithm")]
    pub signing_algorithm: String,
    pub valid: bool,
}
