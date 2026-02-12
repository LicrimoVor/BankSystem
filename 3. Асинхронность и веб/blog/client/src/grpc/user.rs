use super::proto::user::*;
use crate::{
    dto,
    grpc::{GrpcState, utils::auth_request},
    types::{Error, user::UserClientTrait},
};
use std::sync::{Arc, Mutex};
use tonic::Request;

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
}

#[async_trait::async_trait]
impl UserClientTrait for UserClient {
    async fn me(&mut self) -> Result<dto::User, Error> {
        let Some(jwt_token) = self.state.lock().unwrap().access_token.clone() else {
            return Err(Error::Unauthenticated);
        };

        Ok(self
            .inner
            .get_me(auth_request(dto::Empty {}, jwt_token))
            .await?
            .into_inner())
    }

    async fn update(
        &mut self,
        username: Option<&str>,
        email: Option<&str>,
        password: Option<&str>,
    ) -> Result<dto::User, Error> {
        if username.is_none() && email.is_none() && password.is_none() {
            return Err(Error::Unauthenticated);
        }
        let Some(jwt_token) = self.state.lock().unwrap().access_token.clone() else {
            return Err(Error::Unauthenticated);
        };

        let data = UpdateMeRequest {
            username: username.map(String::from),
            email: email.map(String::from),
            password: password.map(String::from),
        };
        Ok(self
            .inner
            .update_me(auth_request(data, jwt_token))
            .await?
            .into_inner())
    }

    async fn delete(&mut self) -> Result<(), Error> {
        let Some(jwt_token) = self.state.lock().unwrap().access_token.clone() else {
            return Err(Error::Unauthenticated);
        };
        self.inner
            .delete_me(auth_request(dto::Empty {}, jwt_token))
            .await?;
        Ok(())
    }

    async fn get_by_email(&mut self, email: &str) -> Result<dto::User, Error> {
        Ok(self
            .inner
            .find_by_email(Request::new(FindByEmailRequest {
                email: email.to_string(),
            }))
            .await?
            .into_inner())
    }
}
