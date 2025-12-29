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
pub struct IamClient {
    inner: NimbusClient,
}

impl IamClient {
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

    /// Adds a role binding to a group.
    pub async fn add_group_role(
        &self,
        params: AddGroupRolePathParams<'_>,
        body: &GroupRoleBindingPayload,
    ) -> Result<GroupRoleBindingResponse, SdkError> {
        let path_params = vec![("group_id", params.group_id.to_string())];
        let result = self
            .inner
            .invoke(&ADD_GROUP_ROLE_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner
            .deserialize::<GroupRoleBindingResponse>(result.body)
    }

    /// Adds a user to a group.
    pub async fn add_user_to_group(
        &self,
        params: AddUserToGroupPathParams<'_>,
        body: &GroupMembershipPayload,
    ) -> Result<GroupMembershipDto, SdkError> {
        let path_params = vec![
            ("tenant", params.tenant.to_string()),
            ("group_id", params.group_id.to_string()),
        ];
        let result = self
            .inner
            .invoke(&ADD_USER_TO_GROUP_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<GroupMembershipDto>(result.body)
    }

    /// Assumes a role and returns temporary session credentials.
    pub async fn assume_role(&self, body: &AssumeRoleRequest) -> Result<Principal, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&ASSUME_ROLE_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<Principal>(result.body)
    }

    /// Attaches a managed policy to a principal.
    pub async fn attach_managed_policy(
        &self,
        params: AttachManagedPolicyPathParams<'_>,
        body: &ManagedPolicyAttachmentPayload,
    ) -> Result<ManagedPolicyResponse, SdkError> {
        let path_params = vec![
            ("tenant", params.tenant.to_string()),
            ("type", params.r#type.to_string()),
            ("id", params.id.to_string()),
        ];
        let result = self
            .inner
            .invoke(&ATTACH_MANAGED_POLICY_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<ManagedPolicyResponse>(result.body)
    }

    /// Attaches a policy to a principal.
    pub async fn attach_policy_to_principal(
        &self,
        params: AttachPolicyToPrincipalPathParams<'_>,
        body: &PolicyAttachmentPayload,
    ) -> Result<Principal, SdkError> {
        let path_params = vec![
            ("tenant", params.tenant.to_string()),
            ("type", params.r#type.to_string()),
            ("id", params.id.to_string()),
        ];
        let result = self
            .inner
            .invoke(
                &ATTACH_POLICY_TO_PRINCIPAL_SPEC,
                &path_params,
                Some(body),
                None,
            )
            .await?;
        self.inner.deserialize::<Principal>(result.body)
    }

    /// Creates a new OIDC provider.
    pub async fn create_oidc_provider(
        &self,
        body: &OidcProviderPayload,
    ) -> Result<OidcProviderResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&CREATE_OIDC_PROVIDER_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<OidcProviderResponse>(result.body)
    }

    /// Creates a new policy within the caller's tenant.
    pub async fn create_policy(&self, body: &CreatePolicy) -> Result<Policy, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&CREATE_POLICY_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<Policy>(result.body)
    }

    /// Creates a new IAM principal.
    pub async fn create_principal(&self, body: &Principal) -> Result<Principal, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&CREATE_PRINCIPAL_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<Principal>(result.body)
    }

    /// Creates a new role.
    pub async fn create_role(&self, body: &RolePayload) -> Result<RoleResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&CREATE_ROLE_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<RoleResponse>(result.body)
    }

    /// Creates a new service account.
    pub async fn create_service_account(
        &self,
        body: &ServiceAccountPayload,
    ) -> Result<ServiceAccountResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&CREATE_SERVICE_ACCOUNT_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner
            .deserialize::<ServiceAccountResponse>(result.body)
    }

    /// Creates a signing key for a tenant.
    pub async fn create_signing_key(
        &self,
        params: CreateSigningKeyPathParams<'_>,
    ) -> Result<SigningKeyResponse, SdkError> {
        let path_params = vec![("tenant", params.tenant.to_string())];
        let result = self
            .inner
            .invoke(&CREATE_SIGNING_KEY_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<SigningKeyResponse>(result.body)
    }

    /// Creates a group for a tenant.
    pub async fn create_tenant_group(
        &self,
        params: CreateTenantGroupPathParams<'_>,
        body: &GroupPayload,
    ) -> Result<GroupDto, SdkError> {
        let path_params = vec![("tenant", params.tenant.to_string())];
        let result = self
            .inner
            .invoke(&CREATE_TENANT_GROUP_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<GroupDto>(result.body)
    }

    /// Creates a user within a tenant.
    pub async fn create_tenant_user(
        &self,
        params: CreateTenantUserPathParams<'_>,
        body: &UserPayload,
    ) -> Result<UserProvisioningResponse, SdkError> {
        let path_params = vec![("tenant", params.tenant.to_string())];
        let result = self
            .inner
            .invoke(&CREATE_TENANT_USER_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner
            .deserialize::<UserProvisioningResponse>(result.body)
    }

    /// Deletes a group.
    pub async fn delete_group(
        &self,
        params: DeleteGroupPathParams<'_>,
    ) -> Result<GroupResponse, SdkError> {
        let path_params = vec![("group_id", params.group_id.to_string())];
        let result = self
            .inner
            .invoke(&DELETE_GROUP_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<GroupResponse>(result.body)
    }

    /// Deletes an OIDC provider.
    pub async fn delete_oidc_provider(
        &self,
        params: DeleteOidcProviderPathParams<'_>,
    ) -> Result<Value, SdkError> {
        let path_params = vec![("provider_id", params.provider_id.to_string())];
        let result = self
            .inner
            .invoke(&DELETE_OIDC_PROVIDER_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<Value>(result.body)
    }

    /// Deletes a policy permanently.
    pub async fn delete_policy(
        &self,
        params: DeletePolicyPathParams<'_>,
    ) -> Result<Value, SdkError> {
        let path_params = vec![
            ("tenant", params.tenant.to_string()),
            ("id", params.id.to_string()),
        ];
        let result = self
            .inner
            .invoke(&DELETE_POLICY_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<Value>(result.body)
    }

    /// Deletes a role.
    pub async fn delete_role(
        &self,
        params: DeleteRolePathParams<'_>,
        body: &RoleQuery,
    ) -> Result<RoleResponse, SdkError> {
        let path_params = vec![("role_id", params.role_id.to_string())];
        let result = self
            .inner
            .invoke(&DELETE_ROLE_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<RoleResponse>(result.body)
    }

    /// Deletes a service account.
    pub async fn delete_service_account(
        &self,
        params: DeleteServiceAccountPathParams<'_>,
        body: &ServiceAccountQuery,
    ) -> Result<ServiceAccountResponse, SdkError> {
        let path_params = vec![("service_account_id", params.service_account_id.to_string())];
        let result = self
            .inner
            .invoke(&DELETE_SERVICE_ACCOUNT_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner
            .deserialize::<ServiceAccountResponse>(result.body)
    }

    /// Deletes a user.
    pub async fn delete_user(
        &self,
        params: DeleteUserPathParams<'_>,
    ) -> Result<UserResponse, SdkError> {
        let path_params = vec![("user_id", params.user_id.to_string())];
        let result = self
            .inner
            .invoke(&DELETE_USER_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<UserResponse>(result.body)
    }

    /// Detaches a managed policy from a principal.
    pub async fn detach_managed_policy(
        &self,
        params: DetachManagedPolicyPathParams<'_>,
    ) -> Result<Value, SdkError> {
        let path_params = vec![
            ("tenant", params.tenant.to_string()),
            ("type", params.r#type.to_string()),
            ("id", params.id.to_string()),
            ("policy_id", params.policy_id.to_string()),
        ];
        let result = self
            .inner
            .invoke(&DETACH_MANAGED_POLICY_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<Value>(result.body)
    }

    /// Detaches a policy from a principal.
    pub async fn detach_policy_from_principal(
        &self,
        params: DetachPolicyFromPrincipalPathParams<'_>,
    ) -> Result<Principal, SdkError> {
        let path_params = vec![
            ("tenant", params.tenant.to_string()),
            ("type", params.r#type.to_string()),
            ("id", params.id.to_string()),
            ("policy_id", params.policy_id.to_string()),
        ];
        let result = self
            .inner
            .invoke(&DETACH_POLICY_FROM_PRINCIPAL_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<Principal>(result.body)
    }

    /// Disables a user login.
    pub async fn disable_user(
        &self,
        params: DisableUserPathParams<'_>,
    ) -> Result<UserResponse, SdkError> {
        let path_params = vec![("user_id", params.user_id.to_string())];
        let result = self
            .inner
            .invoke(&DISABLE_USER_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<UserResponse>(result.body)
    }

    /// Issues a signed token for the requested principal.
    pub async fn emit_token(&self, body: &TokenRequest) -> Result<TokenResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&EMIT_TOKEN_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<TokenResponse>(result.body)
    }

    /// Enables a user login.
    pub async fn enable_user(
        &self,
        params: EnableUserPathParams<'_>,
    ) -> Result<UserResponse, SdkError> {
        let path_params = vec![("user_id", params.user_id.to_string())];
        let result = self
            .inner
            .invoke(&ENABLE_USER_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<UserResponse>(result.body)
    }

    /// Exports audit events for the current tenant.
    pub async fn export_audit_events(
        &self,
        body: &AuditExportQuery,
    ) -> Result<AuditExportJobResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&EXPORT_AUDIT_EVENTS_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner
            .deserialize::<AuditExportJobResponse>(result.body)
    }

    /// Loads a single audit event by identifier.
    pub async fn get_audit_event(
        &self,
        params: GetAuditEventPathParams<'_>,
    ) -> Result<AuditEventResponse, SdkError> {
        let path_params = vec![("event_id", params.event_id.to_string())];
        let result = self
            .inner
            .invoke(&GET_AUDIT_EVENT_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<AuditEventResponse>(result.body)
    }

    /// Loads a group by identifier.
    pub async fn get_group(
        &self,
        params: GetGroupPathParams<'_>,
    ) -> Result<GroupResponse, SdkError> {
        let path_params = vec![("group_id", params.group_id.to_string())];
        let result = self
            .inner
            .invoke(&GET_GROUP_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<GroupResponse>(result.body)
    }

    /// Loads a single managed policy by identifier.
    pub async fn get_managed_policy(
        &self,
        params: GetManagedPolicyPathParams<'_>,
    ) -> Result<ManagedPolicyResponse, SdkError> {
        let path_params = vec![("policy_id", params.policy_id.to_string())];
        let result = self
            .inner
            .invoke(&GET_MANAGED_POLICY_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<ManagedPolicyResponse>(result.body)
    }

    /// Loads an OIDC provider by identifier.
    pub async fn get_oidc_provider(
        &self,
        params: GetOidcProviderPathParams<'_>,
    ) -> Result<OidcProviderResponse, SdkError> {
        let path_params = vec![("provider_id", params.provider_id.to_string())];
        let result = self
            .inner
            .invoke(&GET_OIDC_PROVIDER_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<OidcProviderResponse>(result.body)
    }

    /// Loads a single policy by identifier.
    pub async fn get_policy(&self, params: GetPolicyPathParams<'_>) -> Result<Policy, SdkError> {
        let path_params = vec![
            ("tenant", params.tenant.to_string()),
            ("id", params.id.to_string()),
        ];
        let result = self
            .inner
            .invoke(&GET_POLICY_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<Policy>(result.body)
    }

    /// Retrieves a principal by tenant, type, and identifier.
    pub async fn get_principal(
        &self,
        params: GetPrincipalPathParams<'_>,
    ) -> Result<Principal, SdkError> {
        let path_params = vec![
            ("tenant", params.tenant.to_string()),
            ("type", params.r#type.to_string()),
            ("id", params.id.to_string()),
        ];
        let result = self
            .inner
            .invoke(&GET_PRINCIPAL_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<Principal>(result.body)
    }

    /// Loads a role by identifier.
    pub async fn get_role(
        &self,
        params: GetRolePathParams<'_>,
        body: &RoleQuery,
    ) -> Result<RoleResponse, SdkError> {
        let path_params = vec![("role_id", params.role_id.to_string())];
        let result = self
            .inner
            .invoke(&GET_ROLE_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<RoleResponse>(result.body)
    }

    /// Loads a service account by identifier.
    pub async fn get_service_account(
        &self,
        params: GetServiceAccountPathParams<'_>,
        body: &ServiceAccountQuery,
    ) -> Result<ServiceAccountResponse, SdkError> {
        let path_params = vec![("service_account_id", params.service_account_id.to_string())];
        let result = self
            .inner
            .invoke(&GET_SERVICE_ACCOUNT_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner
            .deserialize::<ServiceAccountResponse>(result.body)
    }

    /// Loads a tenant by identifier.
    pub async fn get_tenant(
        &self,
        params: GetTenantPathParams<'_>,
    ) -> Result<TenantResponse, SdkError> {
        let path_params = vec![("tenant_id", params.tenant_id.to_string())];
        let result = self
            .inner
            .invoke(&GET_TENANT_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<TenantResponse>(result.body)
    }

    /// Loads a user by identifier.
    pub async fn get_user(&self, params: GetUserPathParams<'_>) -> Result<UserResponse, SdkError> {
        let path_params = vec![("user_id", params.user_id.to_string())];
        let result = self
            .inner
            .invoke(&GET_USER_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<UserResponse>(result.body)
    }

    /// Invites a new user to the account.
    pub async fn invite_user(&self, body: &UserInvitePayload) -> Result<UserResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&INVITE_USER_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<UserResponse>(result.body)
    }

    /// Lists audit events for the current tenant.
    pub async fn list_audit_events(
        &self,
        body: &AuditEventQuery,
    ) -> Result<AuditEventListResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&LIST_AUDIT_EVENTS_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner
            .deserialize::<AuditEventListResponse>(result.body)
    }

    /// Lists group members.
    pub async fn list_group_members(
        &self,
        params: ListGroupMembersPathParams<'_>,
    ) -> Result<GroupMembershipListResponse, SdkError> {
        let path_params = vec![("group_id", params.group_id.to_string())];
        let result = self
            .inner
            .invoke(&LIST_GROUP_MEMBERS_SPEC, &path_params, None, None)
            .await?;
        self.inner
            .deserialize::<GroupMembershipListResponse>(result.body)
    }

    /// Lists role bindings for a group.
    pub async fn list_group_roles(
        &self,
        params: ListGroupRolesPathParams<'_>,
    ) -> Result<GroupRoleBindingListResponse, SdkError> {
        let path_params = vec![("group_id", params.group_id.to_string())];
        let result = self
            .inner
            .invoke(&LIST_GROUP_ROLES_SPEC, &path_params, None, None)
            .await?;
        self.inner
            .deserialize::<GroupRoleBindingListResponse>(result.body)
    }

    /// Lists groups for the current account.
    pub async fn list_groups(&self) -> Result<GroupListResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&LIST_GROUPS_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<GroupListResponse>(result.body)
    }

    /// Lists available managed policies.
    pub async fn list_managed_policies(&self) -> Result<ManagedPolicyListResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&LIST_MANAGED_POLICIES_SPEC, &path_params, None, None)
            .await?;
        self.inner
            .deserialize::<ManagedPolicyListResponse>(result.body)
    }

    /// Lists OIDC providers for the account.
    pub async fn list_oidc_providers(
        &self,
        body: &OidcProviderQuery,
    ) -> Result<OidcProviderListResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&LIST_OIDC_PROVIDERS_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner
            .deserialize::<OidcProviderListResponse>(result.body)
    }

    /// Lists policies for the resolved tenant.
    pub async fn list_policies(
        &self,
        body: &PolicyListQuery,
    ) -> Result<PolicyListResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&LIST_POLICIES_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<PolicyListResponse>(result.body)
    }

    /// Lists versions for the specified policy.
    pub async fn list_policy_versions(
        &self,
        params: ListPolicyVersionsPathParams<'_>,
    ) -> Result<PolicyVersionListResponse, SdkError> {
        let path_params = vec![
            ("tenant", params.tenant.to_string()),
            ("policy_id", params.policy_id.to_string()),
        ];
        let result = self
            .inner
            .invoke(&LIST_POLICY_VERSIONS_SPEC, &path_params, None, None)
            .await?;
        self.inner
            .deserialize::<PolicyVersionListResponse>(result.body)
    }

    /// Lists managed policies attached to a principal.
    pub async fn list_principal_managed_policies(
        &self,
        params: ListPrincipalManagedPoliciesPathParams<'_>,
    ) -> Result<ManagedPolicyAttachmentListResponse, SdkError> {
        let path_params = vec![
            ("tenant", params.tenant.to_string()),
            ("type", params.r#type.to_string()),
            ("id", params.id.to_string()),
        ];
        let result = self
            .inner
            .invoke(
                &LIST_PRINCIPAL_MANAGED_POLICIES_SPEC,
                &path_params,
                None,
                None,
            )
            .await?;
        self.inner
            .deserialize::<ManagedPolicyAttachmentListResponse>(result.body)
    }

    /// Lists roles for the account.
    pub async fn list_roles(&self, body: &RoleQuery) -> Result<RoleListResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&LIST_ROLES_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<RoleListResponse>(result.body)
    }

    /// Lists service accounts for the current account.
    pub async fn list_service_accounts(
        &self,
        body: &ServiceAccountQuery,
    ) -> Result<ServiceAccountListResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&LIST_SERVICE_ACCOUNTS_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner
            .deserialize::<ServiceAccountListResponse>(result.body)
    }

    /// Lists console sessions for the account.
    pub async fn list_sessions(
        &self,
        body: &SessionListQuery,
    ) -> Result<SessionListResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&LIST_SESSIONS_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<SessionListResponse>(result.body)
    }

    /// Lists tenants for the account.
    pub async fn list_tenants(&self) -> Result<TenantListResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&LIST_TENANTS_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<TenantListResponse>(result.body)
    }

    /// Lists sessions for a user.
    pub async fn list_user_sessions(
        &self,
        params: ListUserSessionsPathParams<'_>,
    ) -> Result<UserSessionListResponse, SdkError> {
        let path_params = vec![("user_id", params.user_id.to_string())];
        let result = self
            .inner
            .invoke(&LIST_USER_SESSIONS_SPEC, &path_params, None, None)
            .await?;
        self.inner
            .deserialize::<UserSessionListResponse>(result.body)
    }

    /// Lists users for the account.
    pub async fn list_users(&self) -> Result<UserListResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&LIST_USERS_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<UserListResponse>(result.body)
    }

    /// Accepts an audit event payload for storage.
    pub async fn publish_audit_event(&self, body: &AuditEventEnvelope) -> Result<Value, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&PUBLISH_AUDIT_EVENT_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<Value>(result.body)
    }

    /// Removes a member from a group.
    pub async fn remove_group_member(
        &self,
        params: RemoveGroupMemberPathParams<'_>,
    ) -> Result<GroupMembershipResponse, SdkError> {
        let path_params = vec![
            ("group_id", params.group_id.to_string()),
            ("user_id", params.user_id.to_string()),
        ];
        let result = self
            .inner
            .invoke(&REMOVE_GROUP_MEMBER_SPEC, &path_params, None, None)
            .await?;
        self.inner
            .deserialize::<GroupMembershipResponse>(result.body)
    }

    /// Removes a role binding from a group.
    pub async fn remove_group_role(
        &self,
        params: RemoveGroupRolePathParams<'_>,
    ) -> Result<GroupRoleBindingResponse, SdkError> {
        let path_params = vec![
            ("group_id", params.group_id.to_string()),
            ("role_id", params.role_id.to_string()),
        ];
        let result = self
            .inner
            .invoke(&REMOVE_GROUP_ROLE_SPEC, &path_params, None, None)
            .await?;
        self.inner
            .deserialize::<GroupRoleBindingResponse>(result.body)
    }

    /// Revokes sessions matching the supplied filters.
    pub async fn revoke_all_sessions(
        &self,
        body: &SessionRevokeAllPayload,
    ) -> Result<Value, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&REVOKE_ALL_SESSIONS_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<Value>(result.body)
    }

    /// Revokes a service account token.
    pub async fn revoke_service_account_token(
        &self,
        params: RevokeServiceAccountTokenPathParams<'_>,
        body: &ServiceAccountTokenRevokePayload,
    ) -> Result<Value, SdkError> {
        let path_params = vec![("service_account_id", params.service_account_id.to_string())];
        let result = self
            .inner
            .invoke(
                &REVOKE_SERVICE_ACCOUNT_TOKEN_SPEC,
                &path_params,
                Some(body),
                None,
            )
            .await?;
        self.inner.deserialize::<Value>(result.body)
    }

    /// Revokes specific sessions.
    pub async fn revoke_sessions(&self, body: &SessionRevokePayload) -> Result<Value, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&REVOKE_SESSIONS_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<Value>(result.body)
    }

    /// Revokes a user session.
    pub async fn revoke_user_session(
        &self,
        params: RevokeUserSessionPathParams<'_>,
    ) -> Result<UserSessionResponse, SdkError> {
        let path_params = vec![
            ("user_id", params.user_id.to_string()),
            ("session_id", params.session_id.to_string()),
        ];
        let result = self
            .inner
            .invoke(&REVOKE_USER_SESSION_SPEC, &path_params, None, None)
            .await?;
        self.inner.deserialize::<UserSessionResponse>(result.body)
    }

    /// Rotates a service account key.
    pub async fn rotate_service_account_key(
        &self,
        params: RotateServiceAccountKeyPathParams<'_>,
        body: &ServiceAccountQuery,
    ) -> Result<ServiceAccountKeyResponse, SdkError> {
        let path_params = vec![("service_account_id", params.service_account_id.to_string())];
        let result = self
            .inner
            .invoke(
                &ROTATE_SERVICE_ACCOUNT_KEY_SPEC,
                &path_params,
                Some(body),
                None,
            )
            .await?;
        self.inner
            .deserialize::<ServiceAccountKeyResponse>(result.body)
    }

    /// Sets MFA requirement for a user.
    pub async fn set_user_mfa_requirement(
        &self,
        params: SetUserMfaRequirementPathParams<'_>,
        body: &UserMfaRequirementPayload,
    ) -> Result<UserDto, SdkError> {
        let path_params = vec![
            ("tenant", params.tenant.to_string()),
            ("user_id", params.user_id.to_string()),
        ];
        let result = self
            .inner
            .invoke(
                &SET_USER_MFA_REQUIREMENT_SPEC,
                &path_params,
                Some(body),
                None,
            )
            .await?;
        self.inner.deserialize::<UserDto>(result.body)
    }

    /// Suspends a tenant.
    pub async fn suspend_tenant(
        &self,
        params: SuspendTenantPathParams<'_>,
        body: &TenantSuspendPayload,
    ) -> Result<TenantResponse, SdkError> {
        let path_params = vec![("tenant_id", params.tenant_id.to_string())];
        let result = self
            .inner
            .invoke(&SUSPEND_TENANT_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<TenantResponse>(result.body)
    }

    /// Updates a group.
    pub async fn update_group(
        &self,
        params: UpdateGroupPathParams<'_>,
        body: &GroupUpdatePayload,
    ) -> Result<GroupResponse, SdkError> {
        let path_params = vec![("group_id", params.group_id.to_string())];
        let result = self
            .inner
            .invoke(&UPDATE_GROUP_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<GroupResponse>(result.body)
    }

    /// Updates an OIDC provider.
    pub async fn update_oidc_provider(
        &self,
        params: UpdateOidcProviderPathParams<'_>,
        body: &OidcProviderUpdatePayload,
    ) -> Result<OidcProviderResponse, SdkError> {
        let path_params = vec![("provider_id", params.provider_id.to_string())];
        let result = self
            .inner
            .invoke(&UPDATE_OIDC_PROVIDER_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<OidcProviderResponse>(result.body)
    }

    /// Updates the attributes and statements attached to a policy.
    pub async fn update_policy(
        &self,
        params: UpdatePolicyPathParams<'_>,
        body: &UpdatePolicy,
    ) -> Result<Policy, SdkError> {
        let path_params = vec![
            ("tenant", params.tenant.to_string()),
            ("id", params.id.to_string()),
        ];
        let result = self
            .inner
            .invoke(&UPDATE_POLICY_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<Policy>(result.body)
    }

    /// Updates a role.
    pub async fn update_role(
        &self,
        params: UpdateRolePathParams<'_>,
        body: &RoleUpdatePayload,
    ) -> Result<RoleResponse, SdkError> {
        let path_params = vec![("role_id", params.role_id.to_string())];
        let result = self
            .inner
            .invoke(&UPDATE_ROLE_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<RoleResponse>(result.body)
    }

    /// Updates a service account.
    pub async fn update_service_account(
        &self,
        params: UpdateServiceAccountPathParams<'_>,
        body: &ServiceAccountUpdatePayload,
    ) -> Result<ServiceAccountResponse, SdkError> {
        let path_params = vec![("service_account_id", params.service_account_id.to_string())];
        let result = self
            .inner
            .invoke(&UPDATE_SERVICE_ACCOUNT_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner
            .deserialize::<ServiceAccountResponse>(result.body)
    }

    /// Updates a tenant.
    pub async fn update_tenant(
        &self,
        params: UpdateTenantPathParams<'_>,
        body: &TenantUpdatePayload,
    ) -> Result<TenantResponse, SdkError> {
        let path_params = vec![("tenant_id", params.tenant_id.to_string())];
        let result = self
            .inner
            .invoke(&UPDATE_TENANT_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<TenantResponse>(result.body)
    }

    /// Updates a user.
    pub async fn update_user(
        &self,
        params: UpdateUserPathParams<'_>,
        body: &UserUpdatePayload,
    ) -> Result<UserResponse, SdkError> {
        let path_params = vec![("user_id", params.user_id.to_string())];
        let result = self
            .inner
            .invoke(&UPDATE_USER_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<UserResponse>(result.body)
    }

    pub fn waiter(&self) -> LroWaiter {
        self.inner.waiter()
    }
}

#[derive(Clone, Debug)]
pub struct AddGroupRolePathParams<'a> {
    pub group_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct AddUserToGroupPathParams<'a> {
    pub tenant: &'a str,
    pub group_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct AttachManagedPolicyPathParams<'a> {
    pub tenant: &'a str,
    pub r#type: &'a str,
    pub id: &'a str,
}

#[derive(Clone, Debug)]
pub struct AttachPolicyToPrincipalPathParams<'a> {
    pub tenant: &'a str,
    pub r#type: &'a str,
    pub id: &'a str,
}

#[derive(Clone, Debug)]
pub struct CreateSigningKeyPathParams<'a> {
    pub tenant: &'a str,
}

#[derive(Clone, Debug)]
pub struct CreateTenantGroupPathParams<'a> {
    pub tenant: &'a str,
}

#[derive(Clone, Debug)]
pub struct CreateTenantUserPathParams<'a> {
    pub tenant: &'a str,
}

#[derive(Clone, Debug)]
pub struct DeleteGroupPathParams<'a> {
    pub group_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct DeleteOidcProviderPathParams<'a> {
    pub provider_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct DeletePolicyPathParams<'a> {
    pub tenant: &'a str,
    pub id: &'a str,
}

#[derive(Clone, Debug)]
pub struct DeleteRolePathParams<'a> {
    pub role_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct DeleteServiceAccountPathParams<'a> {
    pub service_account_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct DeleteUserPathParams<'a> {
    pub user_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct DetachManagedPolicyPathParams<'a> {
    pub tenant: &'a str,
    pub r#type: &'a str,
    pub id: &'a str,
    pub policy_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct DetachPolicyFromPrincipalPathParams<'a> {
    pub tenant: &'a str,
    pub r#type: &'a str,
    pub id: &'a str,
    pub policy_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct DisableUserPathParams<'a> {
    pub user_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct EnableUserPathParams<'a> {
    pub user_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct GetAuditEventPathParams<'a> {
    pub event_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct GetGroupPathParams<'a> {
    pub group_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct GetManagedPolicyPathParams<'a> {
    pub policy_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct GetOidcProviderPathParams<'a> {
    pub provider_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct GetPolicyPathParams<'a> {
    pub tenant: &'a str,
    pub id: &'a str,
}

#[derive(Clone, Debug)]
pub struct GetPrincipalPathParams<'a> {
    pub tenant: &'a str,
    pub r#type: &'a str,
    pub id: &'a str,
}

#[derive(Clone, Debug)]
pub struct GetRolePathParams<'a> {
    pub role_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct GetServiceAccountPathParams<'a> {
    pub service_account_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct GetTenantPathParams<'a> {
    pub tenant_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct GetUserPathParams<'a> {
    pub user_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct ListGroupMembersPathParams<'a> {
    pub group_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct ListGroupRolesPathParams<'a> {
    pub group_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct ListPolicyVersionsPathParams<'a> {
    pub tenant: &'a str,
    pub policy_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct ListPrincipalManagedPoliciesPathParams<'a> {
    pub tenant: &'a str,
    pub r#type: &'a str,
    pub id: &'a str,
}

#[derive(Clone, Debug)]
pub struct ListUserSessionsPathParams<'a> {
    pub user_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct RemoveGroupMemberPathParams<'a> {
    pub group_id: &'a str,
    pub user_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct RemoveGroupRolePathParams<'a> {
    pub group_id: &'a str,
    pub role_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct RevokeServiceAccountTokenPathParams<'a> {
    pub service_account_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct RevokeUserSessionPathParams<'a> {
    pub user_id: &'a str,
    pub session_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct RotateServiceAccountKeyPathParams<'a> {
    pub service_account_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct SetUserMfaRequirementPathParams<'a> {
    pub tenant: &'a str,
    pub user_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct SuspendTenantPathParams<'a> {
    pub tenant_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct UpdateGroupPathParams<'a> {
    pub group_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct UpdateOidcProviderPathParams<'a> {
    pub provider_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct UpdatePolicyPathParams<'a> {
    pub tenant: &'a str,
    pub id: &'a str,
}

#[derive(Clone, Debug)]
pub struct UpdateRolePathParams<'a> {
    pub role_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct UpdateServiceAccountPathParams<'a> {
    pub service_account_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct UpdateTenantPathParams<'a> {
    pub tenant_id: &'a str,
}

#[derive(Clone, Debug)]
pub struct UpdateUserPathParams<'a> {
    pub user_id: &'a str,
}

const ADD_GROUP_ROLE_SPEC: OperationSpec = OperationSpec {
    name: "AddGroupRole",
    method: SdkHttpMethod::Post,
    uri: "/iam/groups/{group_id}/roles",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const ADD_USER_TO_GROUP_SPEC: OperationSpec = OperationSpec {
    name: "AddUserToGroup",
    method: SdkHttpMethod::Post,
    uri: "/iam/tenants/{tenant}/groups/{group_id}/users",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: true,
    pagination: None,
    lro: false,
};

const ASSUME_ROLE_SPEC: OperationSpec = OperationSpec {
    name: "AssumeRole",
    method: SdkHttpMethod::Post,
    uri: "/iam/assume-role",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: true,
    pagination: None,
    lro: false,
};

const ATTACH_MANAGED_POLICY_SPEC: OperationSpec = OperationSpec {
    name: "AttachManagedPolicy",
    method: SdkHttpMethod::Post,
    uri: "/iam/tenants/{tenant}/principals/{type}/{id}/managed-policies",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: true,
    pagination: None,
    lro: false,
};

const ATTACH_POLICY_TO_PRINCIPAL_SPEC: OperationSpec = OperationSpec {
    name: "AttachPolicyToPrincipal",
    method: SdkHttpMethod::Post,
    uri: "/iam/tenants/{tenant}/principals/{type}/{id}/policies",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: true,
    pagination: None,
    lro: false,
};

const CREATE_OIDC_PROVIDER_SPEC: OperationSpec = OperationSpec {
    name: "CreateOidcProvider",
    method: SdkHttpMethod::Post,
    uri: "/iam/oidc/providers",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const CREATE_POLICY_SPEC: OperationSpec = OperationSpec {
    name: "CreatePolicy",
    method: SdkHttpMethod::Post,
    uri: "/iam/policies",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: true,
    pagination: None,
    lro: false,
};

const CREATE_PRINCIPAL_SPEC: OperationSpec = OperationSpec {
    name: "CreatePrincipal",
    method: SdkHttpMethod::Post,
    uri: "/iam/principals",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: true,
    pagination: None,
    lro: false,
};

const CREATE_ROLE_SPEC: OperationSpec = OperationSpec {
    name: "CreateRole",
    method: SdkHttpMethod::Post,
    uri: "/iam/roles",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const CREATE_SERVICE_ACCOUNT_SPEC: OperationSpec = OperationSpec {
    name: "CreateServiceAccount",
    method: SdkHttpMethod::Post,
    uri: "/iam/service-accounts",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const CREATE_SIGNING_KEY_SPEC: OperationSpec = OperationSpec {
    name: "CreateSigningKey",
    method: SdkHttpMethod::Post,
    uri: "/iam/tenants/{tenant}/signing-keys",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: true,
    pagination: None,
    lro: false,
};

const CREATE_TENANT_GROUP_SPEC: OperationSpec = OperationSpec {
    name: "CreateTenantGroup",
    method: SdkHttpMethod::Post,
    uri: "/iam/tenants/{tenant}/groups",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: true,
    pagination: None,
    lro: false,
};

const CREATE_TENANT_USER_SPEC: OperationSpec = OperationSpec {
    name: "CreateTenantUser",
    method: SdkHttpMethod::Post,
    uri: "/iam/tenants/{tenant}/users",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: true,
    pagination: None,
    lro: false,
};

const DELETE_GROUP_SPEC: OperationSpec = OperationSpec {
    name: "DeleteGroup",
    method: SdkHttpMethod::Delete,
    uri: "/iam/groups/{group_id}",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const DELETE_OIDC_PROVIDER_SPEC: OperationSpec = OperationSpec {
    name: "DeleteOidcProvider",
    method: SdkHttpMethod::Delete,
    uri: "/iam/oidc/providers/{provider_id}",
    success_code: 204,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const DELETE_POLICY_SPEC: OperationSpec = OperationSpec {
    name: "DeletePolicy",
    method: SdkHttpMethod::Delete,
    uri: "/iam/policies/{tenant}/{id}",
    success_code: 204,
    additional_success_responses: &[],
    idempotent: true,
    pagination: None,
    lro: false,
};

const DELETE_ROLE_SPEC: OperationSpec = OperationSpec {
    name: "DeleteRole",
    method: SdkHttpMethod::Delete,
    uri: "/iam/roles/{role_id}",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const DELETE_SERVICE_ACCOUNT_SPEC: OperationSpec = OperationSpec {
    name: "DeleteServiceAccount",
    method: SdkHttpMethod::Delete,
    uri: "/iam/service-accounts/{service_account_id}",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const DELETE_USER_SPEC: OperationSpec = OperationSpec {
    name: "DeleteUser",
    method: SdkHttpMethod::Delete,
    uri: "/iam/users/{user_id}",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const DETACH_MANAGED_POLICY_SPEC: OperationSpec = OperationSpec {
    name: "DetachManagedPolicy",
    method: SdkHttpMethod::Delete,
    uri: "/iam/tenants/{tenant}/principals/{type}/{id}/managed-policies/{policy_id}",
    success_code: 204,
    additional_success_responses: &[],
    idempotent: true,
    pagination: None,
    lro: false,
};

const DETACH_POLICY_FROM_PRINCIPAL_SPEC: OperationSpec = OperationSpec {
    name: "DetachPolicyFromPrincipal",
    method: SdkHttpMethod::Delete,
    uri: "/iam/tenants/{tenant}/principals/{type}/{id}/policies/{policy_id}",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: true,
    pagination: None,
    lro: false,
};

const DISABLE_USER_SPEC: OperationSpec = OperationSpec {
    name: "DisableUser",
    method: SdkHttpMethod::Post,
    uri: "/iam/users/{user_id}/disable",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const EMIT_TOKEN_SPEC: OperationSpec = OperationSpec {
    name: "EmitToken",
    method: SdkHttpMethod::Post,
    uri: "/iam/token",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: true,
    pagination: None,
    lro: false,
};

const ENABLE_USER_SPEC: OperationSpec = OperationSpec {
    name: "EnableUser",
    method: SdkHttpMethod::Post,
    uri: "/iam/users/{user_id}/enable",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const EXPORT_AUDIT_EVENTS_SPEC: OperationSpec = OperationSpec {
    name: "ExportAuditEvents",
    method: SdkHttpMethod::Get,
    uri: "/iam/audit/exports",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const GET_AUDIT_EVENT_SPEC: OperationSpec = OperationSpec {
    name: "GetAuditEvent",
    method: SdkHttpMethod::Get,
    uri: "/iam/audit/events/{event_id}",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const GET_GROUP_SPEC: OperationSpec = OperationSpec {
    name: "GetGroup",
    method: SdkHttpMethod::Get,
    uri: "/iam/groups/{group_id}",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const GET_MANAGED_POLICY_SPEC: OperationSpec = OperationSpec {
    name: "GetManagedPolicy",
    method: SdkHttpMethod::Get,
    uri: "/iam/managed-policies/{policy_id}",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const GET_OIDC_PROVIDER_SPEC: OperationSpec = OperationSpec {
    name: "GetOidcProvider",
    method: SdkHttpMethod::Get,
    uri: "/iam/oidc/providers/{provider_id}",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const GET_POLICY_SPEC: OperationSpec = OperationSpec {
    name: "GetPolicy",
    method: SdkHttpMethod::Get,
    uri: "/iam/policies/{tenant}/{id}",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const GET_PRINCIPAL_SPEC: OperationSpec = OperationSpec {
    name: "GetPrincipal",
    method: SdkHttpMethod::Get,
    uri: "/iam/principals/{tenant}/{type}/{id}",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const GET_ROLE_SPEC: OperationSpec = OperationSpec {
    name: "GetRole",
    method: SdkHttpMethod::Get,
    uri: "/iam/roles/{role_id}",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const GET_SERVICE_ACCOUNT_SPEC: OperationSpec = OperationSpec {
    name: "GetServiceAccount",
    method: SdkHttpMethod::Get,
    uri: "/iam/service-accounts/{service_account_id}",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const GET_TENANT_SPEC: OperationSpec = OperationSpec {
    name: "GetTenant",
    method: SdkHttpMethod::Get,
    uri: "/iam/tenants/{tenant_id}",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const GET_USER_SPEC: OperationSpec = OperationSpec {
    name: "GetUser",
    method: SdkHttpMethod::Get,
    uri: "/iam/users/{user_id}",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const INVITE_USER_SPEC: OperationSpec = OperationSpec {
    name: "InviteUser",
    method: SdkHttpMethod::Post,
    uri: "/iam/users/invite",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const LIST_AUDIT_EVENTS_SPEC: OperationSpec = OperationSpec {
    name: "ListAuditEvents",
    method: SdkHttpMethod::Get,
    uri: "/iam/audit/events",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const LIST_GROUP_MEMBERS_SPEC: OperationSpec = OperationSpec {
    name: "ListGroupMembers",
    method: SdkHttpMethod::Get,
    uri: "/iam/groups/{group_id}/members",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const LIST_GROUP_ROLES_SPEC: OperationSpec = OperationSpec {
    name: "ListGroupRoles",
    method: SdkHttpMethod::Get,
    uri: "/iam/groups/{group_id}/roles",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const LIST_GROUPS_SPEC: OperationSpec = OperationSpec {
    name: "ListGroups",
    method: SdkHttpMethod::Get,
    uri: "/iam/groups",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const LIST_MANAGED_POLICIES_SPEC: OperationSpec = OperationSpec {
    name: "ListManagedPolicies",
    method: SdkHttpMethod::Get,
    uri: "/iam/managed-policies",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const LIST_OIDC_PROVIDERS_SPEC: OperationSpec = OperationSpec {
    name: "ListOidcProviders",
    method: SdkHttpMethod::Get,
    uri: "/iam/oidc/providers",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const LIST_POLICIES_SPEC: OperationSpec = OperationSpec {
    name: "ListPolicies",
    method: SdkHttpMethod::Get,
    uri: "/iam/policies",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const LIST_POLICY_VERSIONS_SPEC: OperationSpec = OperationSpec {
    name: "ListPolicyVersions",
    method: SdkHttpMethod::Get,
    uri: "/iam/policies/{tenant}/{policy_id}/versions",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const LIST_PRINCIPAL_MANAGED_POLICIES_SPEC: OperationSpec = OperationSpec {
    name: "ListPrincipalManagedPolicies",
    method: SdkHttpMethod::Get,
    uri: "/iam/tenants/{tenant}/principals/{type}/{id}/managed-policies",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const LIST_ROLES_SPEC: OperationSpec = OperationSpec {
    name: "ListRoles",
    method: SdkHttpMethod::Get,
    uri: "/iam/roles",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const LIST_SERVICE_ACCOUNTS_SPEC: OperationSpec = OperationSpec {
    name: "ListServiceAccounts",
    method: SdkHttpMethod::Get,
    uri: "/iam/service-accounts",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const LIST_SESSIONS_SPEC: OperationSpec = OperationSpec {
    name: "ListSessions",
    method: SdkHttpMethod::Get,
    uri: "/iam/sessions",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const LIST_TENANTS_SPEC: OperationSpec = OperationSpec {
    name: "ListTenants",
    method: SdkHttpMethod::Get,
    uri: "/iam/tenants",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const LIST_USER_SESSIONS_SPEC: OperationSpec = OperationSpec {
    name: "ListUserSessions",
    method: SdkHttpMethod::Get,
    uri: "/iam/users/{user_id}/sessions",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const LIST_USERS_SPEC: OperationSpec = OperationSpec {
    name: "ListUsers",
    method: SdkHttpMethod::Get,
    uri: "/iam/users",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const PUBLISH_AUDIT_EVENT_SPEC: OperationSpec = OperationSpec {
    name: "PublishAuditEvent",
    method: SdkHttpMethod::Post,
    uri: "/iam/audit/events",
    success_code: 202,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: true,
};

const REMOVE_GROUP_MEMBER_SPEC: OperationSpec = OperationSpec {
    name: "RemoveGroupMember",
    method: SdkHttpMethod::Delete,
    uri: "/iam/groups/{group_id}/members/{user_id}",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const REMOVE_GROUP_ROLE_SPEC: OperationSpec = OperationSpec {
    name: "RemoveGroupRole",
    method: SdkHttpMethod::Delete,
    uri: "/iam/groups/{group_id}/roles/{role_id}",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const REVOKE_ALL_SESSIONS_SPEC: OperationSpec = OperationSpec {
    name: "RevokeAllSessions",
    method: SdkHttpMethod::Post,
    uri: "/iam/sessions/revoke-all",
    success_code: 204,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const REVOKE_SERVICE_ACCOUNT_TOKEN_SPEC: OperationSpec = OperationSpec {
    name: "RevokeServiceAccountToken",
    method: SdkHttpMethod::Post,
    uri: "/iam/service-accounts/{service_account_id}/tokens/revoke",
    success_code: 204,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const REVOKE_SESSIONS_SPEC: OperationSpec = OperationSpec {
    name: "RevokeSessions",
    method: SdkHttpMethod::Post,
    uri: "/iam/sessions/revoke",
    success_code: 204,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const REVOKE_USER_SESSION_SPEC: OperationSpec = OperationSpec {
    name: "RevokeUserSession",
    method: SdkHttpMethod::Delete,
    uri: "/iam/users/{user_id}/sessions/{session_id}",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const ROTATE_SERVICE_ACCOUNT_KEY_SPEC: OperationSpec = OperationSpec {
    name: "RotateServiceAccountKey",
    method: SdkHttpMethod::Post,
    uri: "/iam/service-accounts/{service_account_id}/keys/rotate",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const SET_USER_MFA_REQUIREMENT_SPEC: OperationSpec = OperationSpec {
    name: "SetUserMfaRequirement",
    method: SdkHttpMethod::Put,
    uri: "/iam/tenants/{tenant}/users/{user_id}/mfa",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: true,
    pagination: None,
    lro: false,
};

const SUSPEND_TENANT_SPEC: OperationSpec = OperationSpec {
    name: "SuspendTenant",
    method: SdkHttpMethod::Post,
    uri: "/iam/tenants/{tenant_id}/suspend",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const UPDATE_GROUP_SPEC: OperationSpec = OperationSpec {
    name: "UpdateGroup",
    method: SdkHttpMethod::Patch,
    uri: "/iam/groups/{group_id}",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const UPDATE_OIDC_PROVIDER_SPEC: OperationSpec = OperationSpec {
    name: "UpdateOidcProvider",
    method: SdkHttpMethod::Patch,
    uri: "/iam/oidc/providers/{provider_id}",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const UPDATE_POLICY_SPEC: OperationSpec = OperationSpec {
    name: "UpdatePolicy",
    method: SdkHttpMethod::Put,
    uri: "/iam/policies/{tenant}/{id}",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: true,
    pagination: None,
    lro: false,
};

const UPDATE_ROLE_SPEC: OperationSpec = OperationSpec {
    name: "UpdateRole",
    method: SdkHttpMethod::Patch,
    uri: "/iam/roles/{role_id}",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const UPDATE_SERVICE_ACCOUNT_SPEC: OperationSpec = OperationSpec {
    name: "UpdateServiceAccount",
    method: SdkHttpMethod::Patch,
    uri: "/iam/service-accounts/{service_account_id}",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const UPDATE_TENANT_SPEC: OperationSpec = OperationSpec {
    name: "UpdateTenant",
    method: SdkHttpMethod::Patch,
    uri: "/iam/tenants/{tenant_id}",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};

const UPDATE_USER_SPEC: OperationSpec = OperationSpec {
    name: "UpdateUser",
    method: SdkHttpMethod::Patch,
    uri: "/iam/users/{user_id}",
    success_code: 200,
    additional_success_responses: &[],
    idempotent: false,
    pagination: None,
    lro: false,
};
