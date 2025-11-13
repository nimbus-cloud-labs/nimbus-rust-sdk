use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::time::sleep;
use uuid::Uuid;

use crate::error::SdkError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OperationHandle {
    pub id: Uuid,
    pub status: OperationStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OperationStatus {
    Pending,
    Succeeded,
    Failed,
}

#[async_trait]
pub trait OperationStatusClient: Send + Sync {
    async fn poll(&self, handle: &OperationHandle) -> Result<OperationHandle, SdkError>;
}

pub struct NoopOperationStatusClient;

#[async_trait]
impl OperationStatusClient for NoopOperationStatusClient {
    async fn poll(&self, handle: &OperationHandle) -> Result<OperationHandle, SdkError> {
        Ok(handle.clone())
    }
}

#[derive(Clone)]
pub struct LroWaiter {
    poller: Arc<dyn OperationStatusClient>,
    interval: Duration,
    max_attempts: u32,
}

impl LroWaiter {
    pub fn new(poller: Arc<dyn OperationStatusClient>) -> Self {
        Self {
            poller,
            interval: Duration::from_secs(2),
            max_attempts: 30,
        }
    }

    pub fn with_backoff(mut self, interval: Duration, max_attempts: u32) -> Self {
        self.interval = interval;
        self.max_attempts = max_attempts;
        self
    }

    pub async fn wait(&self, handle: &OperationHandle) -> Result<OperationHandle, SdkError> {
        let mut attempts = 0;
        loop {
            let current = self.poller.poll(handle).await?;
            match current.status {
                OperationStatus::Pending => {
                    attempts += 1;
                    if attempts >= self.max_attempts {
                        return Err(SdkError::OperationTimeout {
                            id: current.id,
                            attempts,
                        });
                    }
                    sleep(self.interval).await;
                }
                _ => return Ok(current),
            }
        }
    }
}

pub fn noop_waiter() -> LroWaiter {
    LroWaiter::new(Arc::new(NoopOperationStatusClient))
}
