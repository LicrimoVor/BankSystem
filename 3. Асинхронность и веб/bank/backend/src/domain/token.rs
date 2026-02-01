use async_trait::async_trait;
use chrono::{DateTime, Utc};
use getset::{Getters, Setters};
use serde::Serialize;
use uuid::Uuid;

use crate::{impl_constructor, infrastructure::error::ErrorApi};

#[derive(Debug, Serialize, Getters, Setters, Clone)]
pub struct RefreshToken {
    #[getset(get = "pub", set = "pub")]
    refresh_token_hash: String,
    #[getset(get = "pub")]
    user_id: Uuid,
    #[getset(get = "pub", set = "pub")]
    expires_at: DateTime<Utc>,
}

impl RefreshToken {
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}

#[async_trait]
pub trait RefreshTokenRepository: Send + Sync {
    async fn create(
        &mut self,
        refresh_token: String,
        user_id: Uuid,
    ) -> Result<RefreshToken, ErrorApi>;
    async fn delete(&mut self, refresh_token: String) -> Result<RefreshToken, ErrorApi>;
    async fn get(&self, refresh_token: String) -> Result<RefreshToken, ErrorApi>;
}

pub mod factory {
    use super::*;
    use crate::infrastructure::security::hash_token;

    pub fn create(
        refresh_token: String,
        user_id: Uuid,
        expires_at: DateTime<Utc>,
    ) -> Result<RefreshToken, ErrorApi> {
        let Ok(refresh_token_hash) = hash_token(&refresh_token) else {
            return Err(ErrorApi::Inner("Hash error".to_string()));
        };

        Ok(RefreshToken {
            user_id,
            expires_at,
            refresh_token_hash,
        })
    }
}
