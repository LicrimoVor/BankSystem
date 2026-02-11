use super::super::{dto, user_service::*};
use crate::{
    application::{auth::AuthService, user::UserService},
    data::Database,
    infrastructure::{config::Config, errors::ErrorBlog},
    preserntation::grpc::{ResultService, extractor::extract_user_id},
};
use std::sync::Arc;
use tonic::{Request, Response};

pub struct UserGRPCSerivce {
    pub database: Arc<Database>,
    pub config: Arc<Config>,
}

#[tonic::async_trait]
impl user_service_server::UserService for UserGRPCSerivce {
    async fn get_me(&self, request: Request<dto::Empty>) -> ResultService<dto::User> {
        let user_id = extract_user_id(&self.config, &request)?;
        let service = UserService(self.database.clone());
        let user = service.get_by_id(user_id).await?;
        Ok(Response::new(user.into()))
    }
    async fn update_me(&self, request: Request<UpdateMeRequest>) -> ResultService<dto::User> {
        let user_id = extract_user_id(&self.config, &request)?;
        let service = UserService(self.database.clone());
        let UpdateMeRequest {
            username,
            email,
            password,
        } = request.into_inner();
        let user = service.update(user_id, username, email, password).await?;
        Ok(Response::new(user.into()))
    }

    async fn delete_me(&self, request: Request<dto::Empty>) -> ResultService<dto::Empty> {
        let user_id = extract_user_id(&self.config, &request)?;
        let service = UserService(self.database.clone());
        service.delete(user_id).await?;
        Ok(dto::Empty {}.into())
    }
    async fn find_by_email(
        &self,
        request: Request<FindByEmailRequest>,
    ) -> ResultService<dto::User> {
        let service = UserService(self.database.clone());
        let FindByEmailRequest { email } = request.into_inner();
        let user = service.get_by_email(email).await?;
        Ok(Response::new(user.into()))
    }
}

pub fn init(
    database: Arc<Database>,
    config: Arc<Config>,
) -> user_service_server::UserServiceServer<UserGRPCSerivce> {
    user_service_server::UserServiceServer::new(UserGRPCSerivce { database, config })
}
