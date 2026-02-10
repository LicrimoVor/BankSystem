use crate::{
    data::Database,
    domain::{
        auth::{JwtToken, RefreshToken},
        user::User,
    },
    infrastructure::{config::Config, errors::ErrorBlog, security::verify_password},
};
use std::sync::Arc;
use uuid::Uuid;

struct AuthService(pub Arc<Database>);

impl AuthService {
    pub async fn login(
        &self,
        config: Arc<Config>,
        email: String,
        password: String,
    ) -> Result<(User, RefreshToken, JwtToken), ErrorBlog> {
        let user_repo = self.0.get_user_repo().await;

        let Some(user) = user_repo.get_by_email(email.clone()).await? else {
            return Err(ErrorBlog::NotFound(format!(
                "User with email {} not found",
                email
            )));
        };
        if !verify_password(password.as_str(), user.password_hash())? {
            return Err(ErrorBlog::Unauthorized("Invalid password".to_string()));
        }
        let mut auth_repo = self.0.get_auth_repo().await;
        let refresh_token = auth_repo.create_refresh_token(user.id().clone()).await?;
        let jwt_token = JwtToken::generate(config.jwt_secret.as_str(), user.id())?;

        Ok((user, refresh_token, jwt_token))
    }

    pub async fn logout(&self, user_id: Uuid, refresh_token: String) -> Result<(), ErrorBlog> {
        let mut auth_repo = self.0.get_auth_repo().await;

        // ТРАНЗАКЦИЯ НАЧАЛО
        let user = auth_repo
            .delete_refresh_token(RefreshToken::from(refresh_token))
            .await?;
        if user != user_id {
            // ТРАНЗАКЦИЯ ОТМЕНЯЕТСЯ
            return Err(ErrorBlog::Unauthorized("Invalid refresh token".to_string()));
        }
        // ТРАНЗАКЦИЯ ОК
        Ok(())
    }

    pub async fn refresh(
        &self,
        config: Arc<Config>,
        refresh_token: String,
    ) -> Result<JwtToken, ErrorBlog> {
        let auth_repo = self.0.get_auth_repo().await;
        let user_id = auth_repo
            .get_refresh_token(RefreshToken::from(refresh_token))
            .await
            .ok_or_else(|| ErrorBlog::Unauthorized("Invalid refresh token".to_string()))?;

        JwtToken::generate(config.jwt_secret.as_str(), &user_id)
    }
}
