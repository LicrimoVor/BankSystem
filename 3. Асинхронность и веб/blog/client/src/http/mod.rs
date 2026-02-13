use crate::types::{
    Client, Error, auth::AuthClientTrait, general::GeneralClientTrait, post::PostClientTrait,
    user::UserClientTrait,
};
use reqwest::{
    Request, RequestBuilder, Response,
    header::{HeaderName, HeaderValue},
};
use std::sync::{Arc, Mutex};
use tonic::client;
mod auth;
mod general;
mod post;
mod user;

pub(self) struct State {
    pub jwt_token: Option<String>,
    pub refresh_token: Option<String>,
    pub csrf: String,
    pub base_url: String,
}

pub const HEADER_CSRF_TOKEN: &'static str = "x-csrf-token";

pub(self) fn url(state: &Arc<Mutex<State>>, path: &str) -> String {
    format!("{}{}", state.lock().unwrap().base_url, path)
}
pub(self) async fn send_csrf(
    state: &Arc<Mutex<State>>,
    req: RequestBuilder,
) -> Result<Response, reqwest::Error> {
    let name = HeaderName::from_static(HEADER_CSRF_TOKEN);
    let value = HeaderValue::from_str(&state.lock().unwrap().csrf).unwrap();
    req.header(name, value).send().await
}
// наверное надо объединить send_csrf и with_auth....
// (но можно не надо)
pub(self) fn with_auth(
    state: &Arc<Mutex<State>>,
    req: RequestBuilder,
) -> Result<RequestBuilder, Error> {
    let guard = state.lock().unwrap();

    if let Some(token) = &guard.jwt_token {
        Ok(req.bearer_auth(token))
    } else {
        Err(Error::Unauthenticated)
    }
}

#[derive(Clone)]
pub struct HttpClient {
    client: reqwest::Client,
    state: Arc<Mutex<State>>,
}

impl HttpClient {
    pub async fn new(addr: &str) -> Result<Self, Error> {
        let client = reqwest::Client::builder().cookie_store(true).build()?;
        let resp = client.get(format!("{addr}/api/health")).send().await?;
        let csrf = resp.cookies().find(|c| c.name() == "csrf-token");
        let Some(csrf) = csrf else {
            return Err(Error::Inner("Not csrf token".to_string()));
        };

        Ok(Self {
            client,
            state: Arc::new(Mutex::new(State {
                jwt_token: None,
                refresh_token: None,
                base_url: format!("{}/api", addr),
                csrf: csrf.value().to_string(),
            })),
        })
    }
}

impl Client for HttpClient {
    fn auth(&self) -> Box<dyn AuthClientTrait> {
        Box::new(auth::AuthClient::new(
            self.client.clone(),
            self.state.clone(),
        ))
    }

    fn general(&self) -> Box<dyn GeneralClientTrait> {
        Box::new(general::GeneralClient::new(
            self.client.clone(),
            self.state.clone(),
        ))
    }

    fn post(&self) -> Box<dyn PostClientTrait> {
        Box::new(post::PostClient::new(
            self.client.clone(),
            self.state.clone(),
        ))
    }

    fn user(&self) -> Box<dyn UserClientTrait> {
        Box::new(user::UserClient::new(
            self.client.clone(),
            self.state.clone(),
        ))
    }
}
