use crate::{
    http::{Error, State, url},
    types::general::GeneralClientTrait,
};
use prost::bytes::Bytes;
use std::sync::{Arc, Mutex};

pub struct GeneralClient {
    client: reqwest::Client,
    state: Arc<Mutex<State>>,
}

impl GeneralClient {
    pub(super) fn new(client: reqwest::Client, state: Arc<Mutex<State>>) -> Self {
        Self { client, state }
    }

    // pub async fn media(&mut self, filename: &str) -> Result<Bytes, Error> {
    //     let res = self
    //         .client
    //         .get(url(&self.state, &format!("/media/{filename}")))
    //         .send()
    //         .await?;

    //     let bytes = res.bytes().await?;
    //     Ok(bytes)
    // }
}

#[async_trait::async_trait]
impl GeneralClientTrait for GeneralClient {
    async fn health(&mut self) -> Result<String, Error> {
        let res = self.client.get(url(&self.state, "/health")).send().await?;
        Ok(res.text().await?)
    }

    async fn ping(&mut self) -> Result<String, Error> {
        let res = self.client.get(url(&self.state, "/ping")).send().await?;
        Ok(res.text().await?)
    }
}
