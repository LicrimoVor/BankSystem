use std::sync::{Arc, Mutex};
use tonic::transport::{Channel, Endpoint};
mod auth;
mod general;
mod post;
mod user;
pub mod utils;
pub mod proto {
    pub mod auth {
        tonic::include_proto!("auth");
    }
    pub mod general {
        tonic::include_proto!("general");
    }
    pub mod post {
        tonic::include_proto!("post");
    }
    pub mod user {
        tonic::include_proto!("user");
    }

    use crate::dto;
}

pub(self) struct GrpcState {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}

pub struct GrpcClient {
    pub channel: Channel,
    state: Arc<Mutex<GrpcState>>,
}

impl GrpcClient {
    pub async fn new(addr: &str) -> Result<Self, tonic::transport::Error> {
        let channel = Endpoint::from_shared(addr.to_string())?.connect().await?;

        Ok(GrpcClient {
            channel,
            state: Arc::new(Mutex::new(GrpcState {
                access_token: None,
                refresh_token: None,
            })),
        })
    }

    pub fn auth(&self) -> auth::AuthClient {
        auth::AuthClient::new(self.channel.clone(), self.state.clone())
    }

    pub fn general(&self) -> general::GeneralClient {
        general::GeneralClient::new(self.channel.clone(), self.state.clone())
    }

    pub fn post(&self) -> post::PostClient {
        post::PostClient::new(self.channel.clone(), self.state.clone())
    }

    pub fn user(&self) -> user::UserClient {
        user::UserClient::new(self.channel.clone(), self.state.clone())
    }
}
