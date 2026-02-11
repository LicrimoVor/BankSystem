use super::super::{auth_service::*, dto};
use crate::{
    application::auth::AuthService,
    data::Database,
    infrastructure::config::Config,
    preserntation::grpc::{ResultService, extractor::extract_user_id},
};
use std::sync::Arc;
use tonic::{Request, Response};

pub struct AuthGRPCSerivce {
    pub database: Arc<Database>,
    pub config: Arc<Config>,
}

#[tonic::async_trait]
impl auth_service_server::AuthService for AuthGRPCSerivce {
    async fn register(&self, request: Request<RegisterRequest>) -> ResultService<RegisterResponse> {
        let RegisterRequest {
            username,
            password,
            email,
        } = request.into_inner();
        let service = AuthService(self.database.clone());
        let (user, refresh, jwt) = service
            .register(self.config.clone(), username, email, password)
            .await?;

        Ok(Response::new(RegisterResponse {
            user: Some(user.into()),
            access_token: jwt.0,
            refresh_token: refresh.0,
        }))
    }
    async fn login(&self, request: Request<LoginRequest>) -> ResultService<LoginResponse> {
        let LoginRequest { email, password } = request.into_inner();
        let service = AuthService(self.database.clone());
        let (user, refresh, jwt) = service.login(self.config.clone(), email, password).await?;

        Ok(Response::new(LoginResponse {
            user: Some(user.into()),
            access_token: jwt.0,
            refresh_token: refresh.0,
        }))
    }

    async fn logout(&self, request: Request<LogoutRequest>) -> ResultService<dto::Empty> {
        let user_id = extract_user_id(&self.config, &request)?;
        let service = AuthService(self.database.clone());
        let LogoutRequest { refresh_token } = request.into_inner();
        service.logout(user_id, refresh_token.into()).await?;
        Ok(dto::Empty {}.into())
    }
    async fn refresh(&self, request: Request<RefreshRequest>) -> ResultService<RefreshResponse> {
        let service = AuthService(self.database.clone());
        let RefreshRequest { refresh_token } = request.into_inner();
        let jwt = service
            .refresh(self.config.clone(), refresh_token.into())
            .await?;
        Ok(Response::new(RefreshResponse {
            access_token: jwt.0,
        }))
    }
}

pub fn init(
    database: Arc<Database>,
    config: Arc<Config>,
) -> auth_service_server::AuthServiceServer<AuthGRPCSerivce> {
    auth_service_server::AuthServiceServer::new(AuthGRPCSerivce { database, config })
}
