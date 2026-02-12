use crate::{
    dto,
    http::{Error, State, send_csrf, url, with_auth},
    types::user::UserClientTrait,
};
use reqwest::Client;
use std::sync::{Arc, Mutex};

pub struct UserClient {
    client: Client,
    state: Arc<Mutex<State>>,
}

impl UserClient {
    pub(super) fn new(client: Client, state: Arc<Mutex<State>>) -> Self {
        Self { client, state }
    }
}

#[async_trait::async_trait]
impl UserClientTrait for UserClient {
    async fn me(&mut self) -> Result<dto::User, Error> {
        let req = self.client.get(url(&self.state, "/user/me"));
        let res = send_csrf(&self.state, with_auth(&self.state, req)?).await?;
        Ok(res.json().await?)
    }

    async fn update(
        &mut self,
        username: Option<&str>,
        email: Option<&str>,
        password: Option<&str>,
    ) -> Result<dto::User, Error> {
        if username.is_none() && email.is_none() && password.is_none() {
            return Err(Error::Inner("empty request".to_string()));
        }
        let payload = serde_json::json!({
            "username": username,
            "email": email,
            "password": password
        });
        let req = self
            .client
            .patch(url(&self.state, "/user/me"))
            .json(&payload);
        let res = send_csrf(&self.state, with_auth(&self.state, req)?).await?;
        Ok(res.json().await?)
    }

    async fn delete(&mut self) -> Result<(), Error> {
        let req = self.client.delete(url(&self.state, "/user/me"));
        send_csrf(&self.state, with_auth(&self.state, req)?).await?;
        Ok(())
    }

    async fn get_by_email(&mut self, email: &str) -> Result<dto::User, Error> {
        let res = self
            .client
            .get(url(&self.state, &format!("/user/{}", email)))
            .send()
            .await?;
        Ok(res.json().await?)
    }
}
