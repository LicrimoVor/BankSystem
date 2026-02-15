use super::proto::auth::*;
use crate::{
    dto,
    grpc::{GrpcState, utils::auth_request},
    types::{Error, auth::AuthClientTrait},
};
use std::sync::{Arc, Mutex};
use tonic::Request;

pub struct AuthClient {
    inner: auth_service_client::AuthServiceClient<tonic::transport::Channel>,
    state: Arc<Mutex<GrpcState>>,
}

impl AuthClient {
    pub(super) fn new(channel: tonic::transport::Channel, state: Arc<Mutex<GrpcState>>) -> Self {
        Self {
            inner: auth_service_client::AuthServiceClient::new(channel),
            state,
        }
    }

    // async fn refresh(&mut self) -> Result<dto::Empty, Status> {
    //     let mut state = self.state.lock().unwrap();
    //     let refresh_token = state.refresh_token.clone();
    //     let Some(refresh_token) = refresh_token else {
    //         return Err(Status::internal("empty token"));
    //     };
    //     let RefreshResponse { access_token } = self
    //         .inner
    //         .refresh(Request::new(RefreshRequest { refresh_token }))
    //         .await?
    //         .into_inner();

    //     state.access_token = Some(access_token);
    //     Ok(dto::Empty {})
    // }
}

#[async_trait::async_trait]
impl AuthClientTrait for AuthClient {
    async fn register(
        &mut self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<dto::User, Error> {
        let RegisterResponse {
            user,
            access_token,
            refresh_token,
        } = self
            .inner
            .register(Request::new(RegisterRequest {
                username: username.to_string(),
                email: email.to_string(),
                password: password.to_string(),
            }))
            .await?
            .into_inner();
        if user.is_none() {
            return Err(Error::Unauthenticated);
        }
        let mut state = self.state.lock().unwrap();
        state.access_token = Some(access_token);
        state.refresh_token = Some(refresh_token);

        Ok(user.unwrap())
    }

    async fn login(&mut self, email: &str, password: &str) -> Result<dto::User, Error> {
        let LoginResponse {
            user,
            access_token,
            refresh_token,
        } = self
            .inner
            .login(Request::new(LoginRequest {
                email: email.to_string(),
                password: password.to_string(),
            }))
            .await?
            .into_inner();
        if user.is_none() {
            return Err(Error::Unauthenticated);
        }
        let mut state = self.state.lock().unwrap();
        state.access_token = Some(access_token);
        state.refresh_token = Some(refresh_token);

        Ok(user.unwrap())
    }

    async fn logout(&mut self) -> Result<(), Error> {
        let refresh_token;
        let jwt_token;
        {
            let state = self.state.lock().unwrap();
            refresh_token = state.refresh_token.clone();
            jwt_token = state.access_token.clone();
        }

        if refresh_token.is_none() || jwt_token.is_none() {
            return Err(Error::Unauthenticated);
        }

        self.inner
            .logout(auth_request(
                LogoutRequest {
                    refresh_token: refresh_token.unwrap(),
                },
                jwt_token.unwrap(),
            ))
            .await?;

        let mut state = self.state.lock().unwrap();
        state.refresh_token = None;
        state.access_token = None;
        Ok(())
    }
}
