//! Generated client – do not edit by hand.
#![allow(clippy::all)]

use crate::types::*;
use nimbus_sdk_core::{
    client::{NimbusClient, OperationSpec, SdkConfig},
    error::SdkError,
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

    /// Assumes a role and returns temporary session credentials.
    pub async fn assume_role(&self, body: &AssumeRoleRequest) -> Result<Principal, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&ASSUME_ROLE_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<Principal>(result.body)
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

    /// Issues a signed token for the requested principal.
    pub async fn emit_token(&self, body: &TokenRequest) -> Result<TokenResponse, SdkError> {
        let path_params: Vec<(&'static str, String)> = Vec::new();
        let result = self
            .inner
            .invoke(&EMIT_TOKEN_SPEC, &path_params, Some(body), None)
            .await?;
        self.inner.deserialize::<TokenResponse>(result.body)
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
}

#[derive(Clone, Debug)]
pub struct DeletePolicyPathParams<'a> {
    pub tenant: &'a str,
    pub id: &'a str,
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
pub struct UpdatePolicyPathParams<'a> {
    pub tenant: &'a str,
    pub id: &'a str,
}

const ASSUME_ROLE_SPEC: OperationSpec = OperationSpec {
    name: "AssumeRole",
    method: SdkHttpMethod::Post,
    uri: "/assume-role",
    success_code: 200,
    idempotent: false,
    pagination: None,
    lro: false,
};

const CREATE_POLICY_SPEC: OperationSpec = OperationSpec {
    name: "CreatePolicy",
    method: SdkHttpMethod::Post,
    uri: "/policies",
    success_code: 200,
    idempotent: false,
    pagination: None,
    lro: false,
};

const CREATE_PRINCIPAL_SPEC: OperationSpec = OperationSpec {
    name: "CreatePrincipal",
    method: SdkHttpMethod::Post,
    uri: "/principals",
    success_code: 200,
    idempotent: false,
    pagination: None,
    lro: false,
};

const DELETE_POLICY_SPEC: OperationSpec = OperationSpec {
    name: "DeletePolicy",
    method: SdkHttpMethod::Delete,
    uri: "/policies/{tenant}/{id}",
    success_code: 204,
    idempotent: false,
    pagination: None,
    lro: false,
};

const EMIT_TOKEN_SPEC: OperationSpec = OperationSpec {
    name: "EmitToken",
    method: SdkHttpMethod::Post,
    uri: "/token",
    success_code: 200,
    idempotent: false,
    pagination: None,
    lro: false,
};

const GET_POLICY_SPEC: OperationSpec = OperationSpec {
    name: "GetPolicy",
    method: SdkHttpMethod::Get,
    uri: "/policies/{tenant}/{id}",
    success_code: 200,
    idempotent: false,
    pagination: None,
    lro: false,
};

const GET_PRINCIPAL_SPEC: OperationSpec = OperationSpec {
    name: "GetPrincipal",
    method: SdkHttpMethod::Get,
    uri: "/principals/{tenant}/{type}/{id}",
    success_code: 200,
    idempotent: false,
    pagination: None,
    lro: false,
};

const UPDATE_POLICY_SPEC: OperationSpec = OperationSpec {
    name: "UpdatePolicy",
    method: SdkHttpMethod::Put,
    uri: "/policies/{tenant}/{id}",
    success_code: 200,
    idempotent: false,
    pagination: None,
    lro: false,
};
