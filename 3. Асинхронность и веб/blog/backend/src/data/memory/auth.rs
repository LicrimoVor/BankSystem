use crate::{
    domain::auth::{AuthRepository, RefreshToken},
    infrastructure::{errors::ErrorBlog, state::State},
};
use std::sync::Arc;
use uuid::Uuid;

pub struct AuthStateRepo(pub Arc<State>);

#[async_trait::async_trait]
impl AuthRepository for AuthStateRepo {
    async fn create_refresh_token(&mut self, user_id: Uuid) -> Result<RefreshToken, ErrorBlog> {
        let mut refresh_token_state = self.0.get_mut_refresh_tokens().await;
        let refresh_token = RefreshToken::generate();
        refresh_token_state.insert(refresh_token.clone(), user_id);
        Ok(refresh_token)
    }

    async fn get_refresh_token(&self, token: RefreshToken) -> Option<Uuid> {
        let refresh_token_state = &self.0.get_refresh_tokens().await;
        refresh_token_state.get(&token).cloned()
    }

    async fn delete_refresh_token(&mut self, token: RefreshToken) -> Result<Uuid, ErrorBlog> {
        let mut refresh_token_state = self.0.get_mut_refresh_tokens().await;
        refresh_token_state
            .remove(&token)
            .ok_or(ErrorBlog::NotFound("Refresh token not found".to_string()))
    }
}
