use crate::{
    dto,
    http::{Error, State, url, with_auth},
    types::auth::AuthClientTrait,
};
use reqwest::Client;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AuthClient {
    client: Client,
    state: Arc<Mutex<State>>,
}

impl AuthClient {
    pub(super) fn new(client: Client, state: Arc<Mutex<State>>) -> Self {
        Self { client, state }
    }
}

#[async_trait::async_trait]
impl AuthClientTrait for AuthClient {
    async fn register(
        &mut self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<dto::User, Error> {
        let payload = serde_json::json!({
            "username": username,
            "email": email,
            "password": password
        });

        let res = self
            .client
            .post(url(&self.state, "/auth/register"))
            .json(&payload)
            .send()
            .await?;

        let body: serde_json::Value = res.json().await?;

        if let Some(jwt) = body.get("access_token").and_then(|v| v.as_str()) {
            self.state.lock().unwrap().jwt_token = Some(jwt.to_string());
        }

        if let Some(refresh) = body.get("refresh_token").and_then(|v| v.as_str()) {
            self.state.lock().unwrap().refresh_token = Some(refresh.to_string());
        }

        let user: dto::User = serde_json::from_value(body["user"].clone())
            .map_err(|_| Error::Inner("Error parse user".to_string()))?;

        Ok(user)
    }

    async fn login(&mut self, email: &str, password: &str) -> Result<dto::User, Error> {
        let payload = serde_json::json!({
            "email": email,
            "password": password
        });

        let res = self
            .client
            .post(url(&self.state, "/auth/login"))
            .json(&payload)
            .send()
            .await?;

        let body: serde_json::Value = res.json().await?;

        if let Some(jwt) = body.get("access_token").and_then(|v| v.as_str()) {
            self.state.lock().unwrap().jwt_token = Some(jwt.to_string());
        }

        if let Some(refresh) = body.get("refresh_token").and_then(|v| v.as_str()) {
            self.state.lock().unwrap().refresh_token = Some(refresh.to_string());
        }
        println!("{}", body["user"].clone());
        let user: dto::User = serde_json::from_value(body["user"].clone())
            .map_err(|_| Error::Inner("Error parse user".to_string()))?;

        Ok(user)
    }

    async fn logout(&mut self) -> Result<(), Error> {
        let payload = {
            let state = self.state.lock().unwrap();
            let refresh_token = state.refresh_token.clone().unwrap_or_default();
            serde_json::json!({ "refresh_token": refresh_token })
        };

        with_auth(
            &self.state,
            self.client.post(url(&self.state, "/auth/logout")),
        )?
        .json(&payload)
        .send()
        .await?;

        let mut state = self.state.lock().unwrap();
        state.jwt_token = None;
        state.refresh_token = None;

        Ok(())
    }
}
