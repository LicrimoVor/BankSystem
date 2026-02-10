use crate::{
    domain::auth::{AuthRepository, RefreshToken},
    infrastructure::{errors::ErrorBlog, state::State},
};
use std::sync::Arc;
use uuid::Uuid;

pub struct AuthPostgresRepo(pub sea_orm::DatabaseConnection);

#[async_trait::async_trait]
impl AuthRepository for AuthPostgresRepo {
    async fn create_refresh_token(&mut self, user_id: Uuid) -> Result<RefreshToken, ErrorBlog> {
        unimplemented!()
    }

    async fn get_refresh_token(&self, token: RefreshToken) -> Option<Uuid> {
        unimplemented!()
    }

    async fn delete_refresh_token(&mut self, token: RefreshToken) -> Result<Uuid, ErrorBlog> {
        unimplemented!()
    }
}
