use async_trait::async_trait;
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    domain::token::{self, RefreshToken, RefreshTokenRepository},
    infrastructure::{error::ErrorApi, security::REFRESH_TOKEN_DURATION, state::State},
};

pub struct RefreshTokenStateRepo(pub Arc<State>);

#[async_trait]
impl RefreshTokenRepository for RefreshTokenStateRepo {
    async fn create(
        &mut self,
        refresh_token_hash: String,
        user_id: Uuid,
    ) -> Result<RefreshToken, ErrorApi> {
        let expires_at = Utc::now() + REFRESH_TOKEN_DURATION;
        let mut refresh_tokens = self.0.refresh_tokens().await;
        let refresh_token =
            token::factory::create(refresh_token_hash.clone(), user_id, expires_at)?;
        refresh_tokens.insert(refresh_token_hash, refresh_token.clone());
        Ok(refresh_token)
    }
    async fn delete(&mut self, refresh_token_hash: String) -> Result<RefreshToken, ErrorApi> {
        let mut refresh_tokens = self.0.refresh_tokens().await;
        let refresh_token = refresh_tokens
            .remove(&refresh_token_hash)
            .ok_or(ErrorApi::NotFound("Refresh token not found".to_string()))?;

        Ok(refresh_token)
    }
    async fn get(&mut self, refresh_token_hash: String) -> Result<RefreshToken, ErrorApi> {
        let refresh_tokens = self.0.refresh_tokens().await;
        let refresh_token = refresh_tokens
            .get(&refresh_token_hash)
            .cloned()
            .ok_or(ErrorApi::NotFound("Refresh token not found".to_string()))?;
        if refresh_token.is_expired() {
            return Err(ErrorApi::Validation("Refresh token expired".to_string()));
        }
        Ok(refresh_token)
    }
}
