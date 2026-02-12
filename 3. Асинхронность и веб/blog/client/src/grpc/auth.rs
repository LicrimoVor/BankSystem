use super::proto::auth::*;
use crate::{
    dto,
    grpc::{GrpcState, utils::auth_request},
};
use std::sync::{Arc, Mutex};
use tonic::{Request, Status};

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

    pub async fn register(
        &mut self,
        username: String,
        email: String,
        password: String,
    ) -> Result<dto::User, Status> {
        let RegisterResponse {
            user,
            access_token,
            refresh_token,
        } = self
            .inner
            .register(Request::new(RegisterRequest {
                username,
                email,
                password,
            }))
            .await?
            .into_inner();
        if user.is_none() {
            return Err(Status::internal("empty token"));
        }
        let mut state = self.state.lock().unwrap();
        state.access_token = Some(access_token);
        state.refresh_token = Some(refresh_token);

        Ok(user.unwrap())
    }

    pub async fn login(&mut self, email: String, password: String) -> Result<dto::User, Status> {
        let LoginResponse {
            user,
            access_token,
            refresh_token,
        } = self
            .inner
            .login(Request::new(LoginRequest { email, password }))
            .await?
            .into_inner();
        if user.is_none() {
            return Err(Status::internal("empty token"));
        }
        let mut state = self.state.lock().unwrap();
        state.access_token = Some(access_token);
        state.refresh_token = Some(refresh_token);

        Ok(user.unwrap())
    }

    async fn refresh(&mut self) -> Result<dto::Empty, Status> {
        let mut state = self.state.lock().unwrap();
        let refresh_token = state.refresh_token.clone();
        let Some(refresh_token) = refresh_token else {
            return Err(Status::internal("empty token"));
        };
        let RefreshResponse { access_token } = self
            .inner
            .refresh(Request::new(RefreshRequest { refresh_token }))
            .await?
            .into_inner();

        state.access_token = Some(access_token);
        Ok(dto::Empty {})
    }

    pub async fn logout(&mut self) -> Result<dto::Empty, Status> {
        let mut state = self.state.lock().unwrap();
        let Some(refresh_token) = state.refresh_token.clone() else {
            return Err(Status::internal("empty token"));
        };
        let Some(jwt_token) = state.access_token.clone() else {
            return Err(Status::internal("empty token"));
        };
        self.inner
            .logout(auth_request(LogoutRequest { refresh_token }, jwt_token))
            .await?;

        state.refresh_token = None;
        state.access_token = None;
        Ok(dto::Empty {})
    }
}
