use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::infrastructure::{
    errors::ErrorBlog,
    security::{Claims, generate_jwt_token, generate_refresh_token, verify_jwt_token},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtToken(String);

impl JwtToken {
    pub fn generate(secret: &str, user_id: &Uuid) -> Result<Self, ErrorBlog> {
        generate_jwt_token(secret, &user_id.to_string()).map(Self)
    }

    pub fn verify(&self) -> Result<Claims, ErrorBlog> {
        verify_jwt_token(&self.0)
    }
}

impl From<String> for JwtToken {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct RefreshToken(String);

impl RefreshToken {
    /// ИСПОЛЬЗОВАТЬ ТОЛЬКО В DATABASE РЕПОЗИТОРИЯХ
    pub fn generate() -> Self {
        Self(generate_refresh_token())
    }

    pub fn verify(&self) -> Result<Claims, ErrorBlog> {
        verify_jwt_token(&self.0)
    }
}

impl From<String> for RefreshToken {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[async_trait::async_trait]
pub trait AuthRepository: Send + Sync {
    async fn create_refresh_token(&mut self, user_id: Uuid) -> Result<RefreshToken, ErrorBlog>;
    async fn get_refresh_token(&self, refresh_token: RefreshToken) -> Option<Uuid>;
    async fn delete_refresh_token(
        &mut self,
        refresh_token: RefreshToken,
    ) -> Result<Uuid, ErrorBlog>;
}
