use crate::{
    domain::token::{RefreshToken, RefreshTokenRepository},
    infrastructure::{
        error::ErrorApi,
        security::{hash_token, REFRESH_TOKEN_DURATION},
    },
};
use async_trait::async_trait;
use chrono::Utc;
use sqlx::PgPool;
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

pub struct RefreshTokenSQLRepo(pub Arc<PgPool>);

#[async_trait]
impl RefreshTokenRepository for RefreshTokenSQLRepo {
    async fn create(
        &mut self,
        refresh_token: String,
        user_id: Uuid,
    ) -> Result<RefreshToken, ErrorApi> {
        todo!()
    }

    async fn delete(&mut self, refresh_token: String) -> Result<RefreshToken, ErrorApi> {
        todo!()
    }

    async fn get(&self, refresh_token: String) -> Result<RefreshToken, ErrorApi> {
        todo!()
    }
}
