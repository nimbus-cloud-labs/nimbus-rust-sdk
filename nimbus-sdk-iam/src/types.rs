//! Generated types – do not edit by hand.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountRegistrationPayload {
    pub slug: String,
    pub name: String,
    #[serde(rename = "adminEmail")]
    pub admin_email: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "adminDisplayName"
    )]
    pub admin_display_name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiKeyListResponse {
    pub keys: ApiKeyListResponseKeysList,
}

pub type ApiKeyListResponseKeysList = Vec<ApiKeyMetadata>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiKeyMetadata {
    #[serde(rename = "keyId")]
    pub key_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub enabled: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiKeyResponse {
    #[serde(rename = "keyId")]
    pub key_id: String,
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    pub secret: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub enabled: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssumeRoleRequest {
    #[serde(rename = "roleUrn")]
    pub role_urn: String,
    #[serde(rename = "sessionName")]
    pub session_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuditEventEnvelope {
    #[serde(rename = "occurredAt")]
    pub occurred_at: String,
    pub event: AuditEventPayload,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuditEventListResponse {
    pub events: AuditEventListResponseEventsList,
}

pub type AuditEventListResponseEventsList = Vec<AuditEventResponse>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuditEventPayload {
    pub r#type: String,
    pub payload: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuditEventQuery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub principal: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuditEventResponse {
    pub id: String,
    pub timestamp: String,
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "tenantSlug")]
    pub tenant_slug: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "accountSlug")]
    pub account_slug: String,
    pub principal: String,
    pub action: String,
    pub resource: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decision: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userAgent")]
    pub user_agent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "prevHash")]
    pub prev_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuditExportJobResponse {
    #[serde(rename = "exportId")]
    pub export_id: String,
    pub format: String,
    pub status: String,
    #[serde(rename = "requestedAt")]
    pub requested_at: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "completedAt"
    )]
    pub completed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<AuditExportJobResponseEventsList>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

pub type AuditExportJobResponseEventsList = Vec<AuditEventResponse>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuditExportQuery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub principal: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConsoleSessionContextUpdatePayload {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tenantId")]
    pub tenant_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "roleId")]
    pub role_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConsoleSessionStartRequest {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ttlSecs")]
    pub ttl_secs: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<ConsoleSessionStartRequestScopeList>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
}

pub type ConsoleSessionStartRequestScopeList = Vec<String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreatePolicy {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    pub statements: CreatePolicyStatementsList,
}

pub type CreatePolicyStatementsList = Vec<Statement>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvaluateRequest {
    pub principal: String,
    pub action: String,
    pub resource: String,
    pub context: Value,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "trustPolicy"
    )]
    pub trust_policy: Option<Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvaluateResponse {
    pub decision: EvaluationDecision,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "matchedStatements")]
    pub matched_statements: EvaluateResponseMatchedStatementsList,
    #[serde(rename = "evaluatedPolicies")]
    pub evaluated_policies: EvaluateResponseEvaluatedPoliciesList,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explain: Option<EvaluationExplanation>,
}

pub type EvaluateResponseEvaluatedPoliciesList = Vec<String>;

pub type EvaluateResponseMatchedStatementsList = Vec<String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvaluationConditionFailure {
    #[serde(rename = "statementId")]
    pub statement_id: String,
    pub failures: EvaluationConditionFailureFailuresList,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvaluationConditionFailureEntry {
    pub operator: String,
    pub failures: EvaluationConditionFailureEntryFailuresList,
}

pub type EvaluationConditionFailureEntryFailuresList = Vec<EvaluationConditionFailureReason>;

pub type EvaluationConditionFailureFailuresList = Vec<EvaluationConditionFailureEntry>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvaluationConditionFailureReason {
    pub key: String,
    pub reason: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum EvaluationDecision {
    Allow,
    Deny,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvaluationExplanation {
    #[serde(rename = "matchedStatements")]
    pub matched_statements: EvaluationExplanationMatchedStatementsList,
    #[serde(rename = "skippedPolicies")]
    pub skipped_policies: EvaluationExplanationSkippedPoliciesList,
    #[serde(rename = "conditionFailures")]
    pub condition_failures: EvaluationExplanationConditionFailuresList,
    #[serde(rename = "explicitDeny")]
    pub explicit_deny: bool,
}

pub type EvaluationExplanationConditionFailuresList = Vec<EvaluationConditionFailure>;

pub type EvaluationExplanationMatchedStatementsList = Vec<String>;

pub type EvaluationExplanationSkippedPoliciesList = Vec<String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvaluationRequest {
    pub principal: String,
    pub action: String,
    pub resource: String,
    pub context: Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<EvaluationRequestScopeList>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "trustPolicy"
    )]
    pub trust_policy: Option<Value>,
}

pub type EvaluationRequestScopeList = Vec<String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvaluationResult {
    pub decision: EvaluationDecision,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "matchedStatements")]
    pub matched_statements: EvaluationResultMatchedStatementsList,
    #[serde(rename = "evaluatedPolicies")]
    pub evaluated_policies: EvaluationResultEvaluatedPoliciesList,
}

pub type EvaluationResultEvaluatedPoliciesList = Vec<String>;

pub type EvaluationResultMatchedStatementsList = Vec<String>;

pub type GroupDto = Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroupListResponse {
    pub groups: GroupListResponseGroupsList,
}

pub type GroupListResponseGroupsList = Vec<GroupResponse>;

pub type GroupMembershipDto = Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroupMembershipListResponse {
    pub members: GroupMembershipListResponseMembersList,
}

pub type GroupMembershipListResponseMembersList = Vec<GroupMembershipResponse>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroupMembershipPayload {
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroupMembershipResponse {
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroupPayload {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroupResponse {
    pub id: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    pub slug: String,
    pub name: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "memberCount")]
    pub member_count: i64,
    #[serde(rename = "roleIds")]
    pub role_ids: GroupResponseRoleIdsList,
}

pub type GroupResponseRoleIdsList = Vec<String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroupRoleBindingListResponse {
    pub roles: GroupRoleBindingListResponseRolesList,
}

pub type GroupRoleBindingListResponseRolesList = Vec<GroupRoleBindingResponse>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroupRoleBindingPayload {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "roleId")]
    pub role_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroupRoleBindingResponse {
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "roleId")]
    pub role_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroupUpdatePayload {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ManagedPolicyAttachmentListResponse {
    pub policies: ManagedPolicyAttachmentListResponsePoliciesList,
}

pub type ManagedPolicyAttachmentListResponsePoliciesList = Vec<ManagedPolicyResponse>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ManagedPolicyAttachmentPayload {
    #[serde(rename = "policyId")]
    pub policy_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ManagedPolicyListResponse {
    pub policies: ManagedPolicyListResponsePoliciesList,
}

pub type ManagedPolicyListResponsePoliciesList = Vec<ManagedPolicyResponse>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ManagedPolicyResponse {
    pub id: String,
    pub name: String,
    pub version: String,
    pub status: Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub statements: ManagedPolicyResponseStatementsList,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

pub type ManagedPolicyResponseStatementsList = Vec<Statement>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OidcProviderListResponse {
    pub providers: OidcProviderListResponseProvidersList,
}

pub type OidcProviderListResponseProvidersList = Vec<OidcProviderResponse>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OidcProviderPayload {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    pub provider: String,
    pub issuer: String,
    #[serde(rename = "authorizationEndpoint")]
    pub authorization_endpoint: String,
    #[serde(rename = "tokenEndpoint")]
    pub token_endpoint: String,
    #[serde(rename = "userinfoEndpoint")]
    pub userinfo_endpoint: String,
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "clientSecret"
    )]
    pub client_secret: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OidcProviderQuery {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tenantId")]
    pub tenant_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OidcProviderResponse {
    #[serde(rename = "providerId")]
    pub provider_id: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    pub provider: String,
    pub issuer: String,
    #[serde(rename = "authorizationEndpoint")]
    pub authorization_endpoint: String,
    #[serde(rename = "tokenEndpoint")]
    pub token_endpoint: String,
    #[serde(rename = "userinfoEndpoint")]
    pub userinfo_endpoint: String,
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "clientSecret"
    )]
    pub client_secret: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OidcProviderUpdatePayload {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tenantId")]
    pub tenant_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "authorizationEndpoint"
    )]
    pub authorization_endpoint: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "tokenEndpoint"
    )]
    pub token_endpoint: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "userinfoEndpoint"
    )]
    pub userinfo_endpoint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientId")]
    pub client_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "clientSecret"
    )]
    pub client_secret: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

pub type Policy = Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PolicyAttachmentPayload {
    #[serde(rename = "policyId")]
    pub policy_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PolicyListQuery {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tenantId")]
    pub tenant_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PolicyListResponse {
    pub policies: PolicyListResponsePoliciesList,
}

pub type PolicyListResponsePoliciesList = Vec<PolicyResponse>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PolicyResponse {
    pub id: String,
    pub name: String,
    pub version: String,
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    pub statements: PolicyResponseStatementsList,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

pub type PolicyResponseStatementsList = Vec<Statement>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PolicyVersionListResponse {
    pub versions: PolicyVersionListResponseVersionsList,
}

pub type PolicyVersionListResponseVersionsList = Vec<PolicyResponse>;

pub type Principal = Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReplicationAccountPayload {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "accountSlug")]
    pub account_slug: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "accountNumber")]
    pub account_number: i64,
    pub status: String,
    #[serde(rename = "loginDisabled")]
    pub login_disabled: bool,
    pub revision: i64,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "originRegion")]
    pub origin_region: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReplicationTenantPayload {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "tenantSlug")]
    pub tenant_slug: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "isPrimary")]
    pub is_primary: bool,
    pub status: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "suspendedAt"
    )]
    pub suspended_at: Option<String>,
    pub revision: i64,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "originRegion")]
    pub origin_region: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RevokeTokenRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jti: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub urn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "expiresAt")]
    pub expires_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoleListResponse {
    pub roles: RoleListResponseRolesList,
}

pub type RoleListResponseRolesList = Vec<RoleResponse>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RolePayload {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "roleId")]
    pub role_id: String,
    #[serde(rename = "policyIds")]
    pub policy_ids: RolePayloadPolicyIdsList,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<RolePayloadAttributesMap>,
}

pub type RolePayloadAttributesMap = BTreeMap<String, String>;

pub type RolePayloadPolicyIdsList = Vec<String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoleQuery {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tenantId")]
    pub tenant_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoleResponse {
    #[serde(rename = "roleId")]
    pub role_id: String,
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "policyIds")]
    pub policy_ids: RoleResponsePolicyIdsList,
    pub attributes: RoleResponseAttributesMap,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

pub type RoleResponseAttributesMap = BTreeMap<String, String>;

pub type RoleResponsePolicyIdsList = Vec<String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoleUpdatePayload {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "policyIds")]
    pub policy_ids: Option<RoleUpdatePayloadPolicyIdsList>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<RoleUpdatePayloadAttributesMap>,
}

pub type RoleUpdatePayloadAttributesMap = BTreeMap<String, String>;

pub type RoleUpdatePayloadPolicyIdsList = Vec<String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServiceAccountListResponse {
    #[serde(rename = "serviceAccounts")]
    pub service_accounts: ServiceAccountListResponseServiceAccountsList,
}

pub type ServiceAccountListResponseServiceAccountsList = Vec<ServiceAccountResponse>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServiceAccountPayload {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "serviceAccountId")]
    pub service_account_id: String,
    #[serde(rename = "policyIds")]
    pub policy_ids: ServiceAccountPayloadPolicyIdsList,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<ServiceAccountPayloadAttributesMap>,
}

pub type ServiceAccountPayloadAttributesMap = BTreeMap<String, String>;

pub type ServiceAccountPayloadPolicyIdsList = Vec<String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServiceAccountQuery {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tenantId")]
    pub tenant_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServiceAccountResponse {
    #[serde(rename = "serviceAccountId")]
    pub service_account_id: String,
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "policyIds")]
    pub policy_ids: ServiceAccountResponsePolicyIdsList,
    pub attributes: ServiceAccountResponseAttributesMap,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "keyEnabled")]
    pub key_enabled: bool,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "keyCreatedAt"
    )]
    pub key_created_at: Option<String>,
}

pub type ServiceAccountResponseAttributesMap = BTreeMap<String, String>;

pub type ServiceAccountResponsePolicyIdsList = Vec<String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServiceAccountTokenRevokePayload {
    pub jti: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "expiresAt")]
    pub expires_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServiceAccountUpdatePayload {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "policyIds")]
    pub policy_ids: Option<ServiceAccountUpdatePayloadPolicyIdsList>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<ServiceAccountUpdatePayloadAttributesMap>,
}

pub type ServiceAccountUpdatePayloadAttributesMap = BTreeMap<String, String>;

pub type ServiceAccountUpdatePayloadPolicyIdsList = Vec<String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionListQuery {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tenantId")]
    pub tenant_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userId")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionListResponse {
    pub sessions: SessionListResponseSessionsList,
}

pub type SessionListResponseSessionsList = Vec<SessionResponse>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionResponse {
    pub id: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "tokenJti")]
    pub token_jti: String,
    #[serde(rename = "issuedAt")]
    pub issued_at: String,
    #[serde(rename = "expiresAt")]
    pub expires_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "revokedAt")]
    pub revoked_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<SessionResponseScopeList>,
    pub status: String,
}

pub type SessionResponseScopeList = Vec<String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionRevokeAllPayload {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tenantId")]
    pub tenant_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userId")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionRevokePayload {
    #[serde(rename = "sessionIds")]
    pub session_ids: SessionRevokePayloadSessionIdsList,
}

pub type SessionRevokePayloadSessionIdsList = Vec<String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SigningKeyResponse {
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "keyId")]
    pub key_id: String,
    pub secret: String,
    pub active: bool,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Statement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<Value>,
    pub action: StatementActionList,
    pub resource: StatementResourceList,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<StatementConditionMap>,
}

pub type StatementActionList = Vec<String>;

pub type StatementConditionMap = BTreeMap<String, Value>;

pub type StatementResourceList = Vec<String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TenantListResponse {
    pub tenants: TenantListResponseTenantsList,
}

pub type TenantListResponseTenantsList = Vec<TenantResponse>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TenantPayload {
    pub slug: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isPrimary")]
    pub is_primary: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TenantResponse {
    pub id: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "accountSlug")]
    pub account_slug: String,
    pub slug: String,
    pub name: String,
    #[serde(rename = "isPrimary")]
    pub is_primary: bool,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "suspendedAt"
    )]
    pub suspended_at: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TenantSecuritySettingsUpdatePayload {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "enforceMfa"
    )]
    pub enforce_mfa: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "allowedIdentityProviders"
    )]
    pub allowed_identity_providers:
        Option<TenantSecuritySettingsUpdatePayloadAllowedIdentityProvidersList>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "trustedDomains"
    )]
    pub trusted_domains: Option<TenantSecuritySettingsUpdatePayloadTrustedDomainsList>,
}

pub type TenantSecuritySettingsUpdatePayloadAllowedIdentityProvidersList = Vec<String>;

pub type TenantSecuritySettingsUpdatePayloadTrustedDomainsList = Vec<String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TenantSuspendPayload {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TenantUpdatePayload {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenRequest {
    pub urn: String,
    pub typ: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<TokenRequestScopeList>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aud: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ttlSecs")]
    pub ttl_secs: Option<i64>,
}

pub type TokenRequestScopeList = Vec<String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub token: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdatePolicy {
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    pub statements: UpdatePolicyStatementsList,
}

pub type UpdatePolicyStatementsList = Vec<Statement>;

pub type UserDto = Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserInvitePayload {
    #[serde(rename = "accountId")]
    pub account_id: String,
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "displayName"
    )]
    pub display_name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserKeyQuery {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tenantId")]
    pub tenant_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserListResponse {
    pub users: UserListResponseUsersList,
}

pub type UserListResponseUsersList = Vec<UserResponse>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserMfaRequirementPayload {
    #[serde(rename = "requiresMfa")]
    pub requires_mfa: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserPayload {
    #[serde(rename = "accountId")]
    pub account_id: String,
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "displayName"
    )]
    pub display_name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserProvisioningResponse {
    pub user: UserDto,
    pub principal: Principal,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    pub email: String,
    #[serde(rename = "loginState")]
    pub login_state: String,
    #[serde(rename = "loginDisabled")]
    pub login_disabled: bool,
    #[serde(rename = "requiresMfa")]
    pub requires_mfa: bool,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "displayName"
    )]
    pub display_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "mfaCompletedAt"
    )]
    pub mfa_completed_at: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserSessionListResponse {
    pub sessions: UserSessionListResponseSessionsList,
}

pub type UserSessionListResponseSessionsList = Vec<UserSessionResponse>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserSessionResponse {
    pub id: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "issuedAt")]
    pub issued_at: String,
    #[serde(rename = "expiresAt")]
    pub expires_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "revokedAt")]
    pub revoked_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<UserSessionResponseScopeList>,
}

pub type UserSessionResponseScopeList = Vec<String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserUpdatePayload {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "displayName"
    )]
    pub display_name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValidateAwsSigV4Request {
    pub method: String,
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    pub headers: ValidateAwsSigV4RequestHeadersList,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization: Option<String>,
}

pub type ValidateAwsSigV4RequestHeadersList = Vec<ValidateHmacHeader>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValidateHmacHeader {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValidateHmacRequest {
    pub signature: String,
    pub date: String,
    pub headers: ValidateHmacRequestHeadersList,
    pub body: String,
    #[serde(rename = "bodyChecksum")]
    pub body_checksum: String,
}

pub type ValidateHmacRequestHeadersList = Vec<ValidateHmacHeader>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValidateJwtRequest {
    pub token: String,
}
