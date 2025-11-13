use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use http::{HeaderMap, HeaderName, HeaderValue};
use nimbus_sdk_compute::ComputeClient;
use nimbus_sdk_core::{
    auth::StaticTokenProvider,
    client::{NimbusClient, SdkConfigBuilder},
    error::SdkError,
    lro::{OperationHandle, OperationStatus, OperationStatusClient},
    transport::{Transport, TransportError, TransportRequest, TransportResponse},
};
use nimbus_sdk_iam::IamClient;
use serde_json::json;
use tokio::time::Duration;
use uuid::Uuid;

#[tokio::test]
async fn emits_token_via_generated_client() {
    let transport = Arc::new(MockTransport::new(vec![MockReply::json(
        200,
        json!({ "token": "demo" }),
    )]));
    let config = SdkConfigBuilder::default()
        .auth(Arc::new(StaticTokenProvider::bearer("unit")))
        .transport(transport)
        .build()
        .unwrap();
    let client = NimbusClient::new(config);
    let iam = IamClient::new(client);

    let response = iam
        .emit_token(&json!({ "urn": "urn:nimbus:iam::123", "typ": "access" }))
        .await
        .unwrap();
    assert_eq!(response["token"], "demo");
}

#[tokio::test]
async fn paginates_over_compute_results() {
    let replies = vec![
        MockReply::with_header(
            200,
            json!({ "items": [1, 2], "next": "cursor-1" }),
            ("Content-Range", "cursor=cursor-1"),
        ),
        MockReply::json(200, json!({ "items": [3, 4] })),
    ];
    let transport = Arc::new(MockTransport::new(replies));
    let config = SdkConfigBuilder::default()
        .auth(Arc::new(StaticTokenProvider::bearer("unit")))
        .transport(transport)
        .build()
        .unwrap();
    let client = NimbusClient::new(config);
    let compute = ComputeClient::new(client);

    let mut paginator = compute.paginate_list_networks();
    let first = paginator.next_page().await.unwrap().unwrap();
    assert_eq!(first["items"][0], 1);
    let second = paginator.next_page().await.unwrap().unwrap();
    assert_eq!(second["items"][0], 3);
    assert!(paginator.next_page().await.unwrap().is_none());
}

#[tokio::test]
async fn waits_for_long_running_operations() {
    let transport = Arc::new(MockTransport::new(vec![MockReply::json(
        202,
        json!({ "id": Uuid::now_v7(), "status": "pending" }),
    )]));
    let poller = Arc::new(MockOperationPoller::new(vec![
        OperationStatus::Pending,
        OperationStatus::Succeeded,
    ]));
    let config = SdkConfigBuilder::default()
        .auth(Arc::new(StaticTokenProvider::bearer("unit")))
        .transport(transport)
        .lro_client(poller)
        .build()
        .unwrap();
    let client = NimbusClient::new(config);
    let compute = ComputeClient::new(client);

    let handle = OperationHandle {
        id: Uuid::now_v7(),
        status: OperationStatus::Pending,
        metadata: None,
        error: None,
    };

    let resolved = compute.waiter().wait(&handle).await.unwrap();
    assert_eq!(resolved.status, OperationStatus::Succeeded);
}

struct MockTransport {
    responses: Mutex<Vec<MockReply>>,
}

impl MockTransport {
    fn new(responses: Vec<MockReply>) -> Self {
        Self {
            responses: Mutex::new(responses),
        }
    }
}

#[async_trait]
impl Transport for MockTransport {
    async fn execute(
        &self,
        _request: TransportRequest,
    ) -> Result<TransportResponse, TransportError> {
        let reply = self.responses.lock().unwrap().remove(0);
        let mut headers = HeaderMap::new();
        if let Some((name, value)) = reply.header {
            let header_name = HeaderName::from_bytes(name.as_bytes()).unwrap();
            let header_value = HeaderValue::from_str(value).unwrap();
            headers.insert(header_name, header_value);
        }
        Ok(TransportResponse {
            status: reply.status,
            headers,
            body: reply.body,
        })
    }
}

struct MockReply {
    status: u16,
    body: serde_json::Value,
    header: Option<(&'static str, &'static str)>,
}

impl MockReply {
    fn json(status: u16, body: serde_json::Value) -> Self {
        Self {
            status,
            body,
            header: None,
        }
    }

    fn with_header(
        status: u16,
        body: serde_json::Value,
        header: (&'static str, &'static str),
    ) -> Self {
        Self {
            status,
            body,
            header: Some(header),
        }
    }
}

struct MockOperationPoller {
    states: Mutex<Vec<OperationStatus>>,
}

impl MockOperationPoller {
    fn new(states: Vec<OperationStatus>) -> Self {
        Self {
            states: Mutex::new(states),
        }
    }
}

#[async_trait]
impl OperationStatusClient for MockOperationPoller {
    async fn poll(&self, handle: &OperationHandle) -> Result<OperationHandle, SdkError> {
        tokio::time::sleep(Duration::from_millis(5)).await;
        let next_status = self.states.lock().unwrap().remove(0);
        Ok(OperationHandle {
            status: next_status,
            ..handle.clone()
        })
    }
}
