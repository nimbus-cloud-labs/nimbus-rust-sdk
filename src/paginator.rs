use std::marker::PhantomData;

use serde::de::DeserializeOwned;

use crate::client::{NimbusClient, OperationSpec};
use crate::error::SdkError;

pub struct Paginator<'client, T> {
    client: &'client NimbusClient,
    spec: &'static OperationSpec,
    path_params: Vec<(&'static str, String)>,
    cursor: Option<String>,
    finished: bool,
    _marker: PhantomData<T>,
}

impl<'client, T> Paginator<'client, T>
where
    T: DeserializeOwned,
{
    pub(crate) fn new(
        client: &'client NimbusClient,
        spec: &'static OperationSpec,
        path_params: Vec<(&'static str, String)>,
    ) -> Self {
        Self {
            client,
            spec,
            path_params,
            cursor: None,
            finished: false,
            _marker: PhantomData,
        }
    }

    pub async fn next_page(&mut self) -> Result<Option<T>, SdkError> {
        if self.finished {
            return Ok(None);
        }
        let result = self
            .client
            .invoke(self.spec, &self.path_params, None, self.cursor.as_deref())
            .await?;
        self.cursor = result.next_cursor.clone();
        if self.cursor.is_none() {
            self.finished = true;
        }
        let value = self.client.deserialize::<T>(result.body)?;
        Ok(Some(value))
    }
}
