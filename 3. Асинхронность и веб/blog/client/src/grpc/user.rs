use super::proto::user::*;
use crate::{
    dto,
    grpc::{GrpcState, utils::auth_request},
};
use std::sync::{Arc, Mutex};
use tonic::{Request, Status};

pub struct UserClient {
    inner: user_service_client::UserServiceClient<tonic::transport::Channel>,
    state: Arc<Mutex<GrpcState>>,
}

impl UserClient {
    pub(super) fn new(channel: tonic::transport::Channel, state: Arc<Mutex<GrpcState>>) -> Self {
        Self {
            inner: user_service_client::UserServiceClient::new(channel),
            state,
        }
    }

    pub async fn me(&mut self) -> Result<dto::User, Status> {
        let Some(jwt_token) = self.state.lock().unwrap().access_token.clone() else {
            return Err(Status::unauthenticated("unauthorized"));
        };

        Ok(self
            .inner
            .get_me(auth_request(dto::Empty {}, jwt_token))
            .await?
            .into_inner())
    }

    pub async fn update(
        &mut self,
        username: Option<String>,
        email: Option<String>,
        password: Option<String>,
    ) -> Result<dto::User, Status> {
        if username.is_none() && email.is_none() && password.is_none() {
            return Err(Status::invalid_argument("empty request"));
        }
        let Some(jwt_token) = self.state.lock().unwrap().access_token.clone() else {
            return Err(Status::unauthenticated("unauthorized"));
        };

        let data = UpdateMeRequest {
            username,
            email,
            password,
        };
        Ok(self
            .inner
            .update_me(auth_request(data, jwt_token))
            .await?
            .into_inner())
    }

    pub async fn delete(&mut self) -> Result<dto::Empty, Status> {
        let Some(jwt_token) = self.state.lock().unwrap().access_token.clone() else {
            return Err(Status::unauthenticated("unauthorized"));
        };
        Ok(self
            .inner
            .delete_me(auth_request(dto::Empty {}, jwt_token))
            .await?
            .into_inner())
    }

    pub async fn find_by_email(&mut self, email: String) -> Result<dto::User, Status> {
        Ok(self
            .inner
            .find_by_email(Request::new(FindByEmailRequest { email }))
            .await?
            .into_inner())
    }
}
