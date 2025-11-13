use std::sync::Arc;

use once_cell::sync::Lazy;
use uuid::Uuid;

pub trait IdempotencyTokenProvider: Send + Sync {
    fn next_token(&self) -> String;
}

#[derive(Default)]
pub struct UuidTokenProvider;

impl IdempotencyTokenProvider for UuidTokenProvider {
    fn next_token(&self) -> String {
        Uuid::now_v7().to_string()
    }
}

pub fn default_provider() -> Arc<dyn IdempotencyTokenProvider> {
    static PROVIDER: Lazy<Arc<dyn IdempotencyTokenProvider>> =
        Lazy::new(|| Arc::new(UuidTokenProvider));
    PROVIDER.clone()
}
