use std::sync::{Arc, Mutex};
use tonic::transport::{Channel, Endpoint};

use crate::types::{
    Client, auth::AuthClientTrait, general::GeneralClientTrait, post::PostClientTrait,
    user::UserClientTrait,
};
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

#[derive(Clone)]
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
}

impl Client for GrpcClient {
    fn auth(&self) -> Box<dyn AuthClientTrait> {
        Box::new(auth::AuthClient::new(
            self.channel.clone(),
            self.state.clone(),
        ))
    }

    fn general(&self) -> Box<dyn GeneralClientTrait> {
        Box::new(general::GeneralClient::new(
            self.channel.clone(),
            self.state.clone(),
        ))
    }

    fn post(&self) -> Box<dyn PostClientTrait> {
        Box::new(post::PostClient::new(
            self.channel.clone(),
            self.state.clone(),
        ))
    }

    fn user(&self) -> Box<dyn UserClientTrait> {
        Box::new(user::UserClient::new(
            self.channel.clone(),
            self.state.clone(),
        ))
    }
}
