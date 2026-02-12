use crate::{
    dto,
    http::{Error, State, send_csrf, url, with_auth},
    types::post::PostClientTrait,
};
use reqwest::Client;
use std::sync::{Arc, Mutex};

pub struct PostClient {
    client: Client,
    state: Arc<Mutex<State>>,
}

impl PostClient {
    pub(super) fn new(client: Client, state: Arc<Mutex<State>>) -> Self {
        Self { client, state }
    }
}

#[async_trait::async_trait]
impl PostClientTrait for PostClient {
    async fn create(
        &mut self,
        title: &str,
        content: &str,
        img_base64: Option<&str>,
    ) -> Result<dto::Post, Error> {
        let payload = serde_json::json!( {
            "title": title,
            "content": content,
            "img_base64": img_base64,
        });

        let req = self.client.post(url(&self.state, "/post")).json(&payload);
        let res = send_csrf(&self.state, with_auth(&self.state, req)?).await?;
        Ok(res.json().await?)
    }

    async fn update(
        &mut self,
        post_id: &str,
        title: Option<&str>,
        content: Option<&str>,
        img_base64: Option<&str>,
    ) -> Result<dto::Post, Error> {
        if title.is_none() && content.is_none() && img_base64.is_none() {
            return Err(Error::Inner("empty request".to_string()));
        }

        let payload = serde_json::json!( {
            "title": title,
            "content": content,
            "img_base64": img_base64,
        });
        let req = self
            .client
            .patch(url(&self.state, &format!("/post/{}", post_id)))
            .json(&payload);

        let res = send_csrf(&self.state, with_auth(&self.state, req)?).await?;
        Ok(res.json().await?)
    }

    async fn delete(&mut self, post_id: &str) -> Result<(), Error> {
        let req = self
            .client
            .delete(url(&self.state, &format!("/post/{}", post_id)));
        send_csrf(&self.state, with_auth(&self.state, req)?).await?;
        Ok(())
    }

    async fn get_by_id(&mut self, post_id: &str) -> Result<dto::Post, Error> {
        let res = self
            .client
            .get(url(&self.state, &format!("/post/{}", post_id)))
            .send()
            .await?;
        Ok(res.json().await?)
    }

    async fn gets_by_author(&mut self, email: &str) -> Result<Vec<dto::Post>, Error> {
        let res = self
            .client
            .get(url(&self.state, &format!("/post/author/{}", email)))
            .send()
            .await?;
        Ok(res.json().await?)
    }

    async fn gets_me(&mut self) -> Result<Vec<dto::Post>, Error> {
        let req = self.client.get(url(&self.state, "/post/me"));
        let res = send_csrf(&self.state, with_auth(&self.state, req)?).await?;
        Ok(res.json().await?)
    }
}
