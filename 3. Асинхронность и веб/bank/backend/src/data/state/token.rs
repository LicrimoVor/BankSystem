use crate::{
    domain::token::{self, RefreshToken, RefreshTokenRepository},
    infrastructure::{
        error::ErrorApi,
        security::{hash_token, REFRESH_TOKEN_DURATION},
        state::State,
    },
};
use async_trait::async_trait;
use chrono::Utc;
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

pub struct RefreshTokenStateRepo(pub Arc<State>);

#[async_trait]
impl RefreshTokenRepository for RefreshTokenStateRepo {
    async fn create(
        &mut self,
        refresh_token: String,
        user_id: Uuid,
    ) -> Result<RefreshToken, ErrorApi> {
        let expires_at = Utc::now() + REFRESH_TOKEN_DURATION;
        let mut refresh_tokens = self.0.refresh_tokens().await;
        let token = token::factory::create(refresh_token, user_id, expires_at)?;
        refresh_tokens.insert(token.refresh_token_hash().clone(), token.clone());
        Ok(token)
    }

    async fn delete(&mut self, refresh_token: String) -> Result<RefreshToken, ErrorApi> {
        let Ok(refresh_token_hash) = hash_token(&refresh_token) else {
            return Err(ErrorApi::Inner("Hash error".to_string()));
        };
        let mut refresh_tokens = self.0.refresh_tokens().await;
        let token = refresh_tokens
            .remove(&refresh_token_hash)
            .ok_or(ErrorApi::NotFound("Refresh token not found".to_string()))?;

        Ok(token)
    }

    async fn get(&self, refresh_token: String) -> Result<RefreshToken, ErrorApi> {
        let Ok(refresh_token_hash) = hash_token(&refresh_token) else {
            return Err(ErrorApi::Inner("Hash error".to_string()));
        };
        let refresh_tokens = self.0.refresh_tokens().await;
        info!("{:#?}", refresh_tokens);
        info!("{:#?} - {:#?}", refresh_token, refresh_token_hash);

        let token = refresh_tokens
            .get(&refresh_token_hash)
            .cloned()
            .ok_or(ErrorApi::NotFound("Refresh token not found".to_string()))?;
        if token.is_expired() {
            return Err(ErrorApi::Validation("Refresh token expired".to_string()));
        }
        Ok(token)
    }
}
