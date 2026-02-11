use crate::{data::Database, domain::user::User, infrastructure::errors::ErrorBlog};
use std::sync::Arc;

pub struct UserService(pub Arc<Database>);

impl UserService {
    pub async fn get_by_id(&self, user_id: uuid::Uuid) -> Result<User, ErrorBlog> {
        let user_repo = self.0.get_user_repo().await;
        user_repo
            .get_by_id(user_id)
            .await?
            .ok_or_else(|| ErrorBlog::NotFound(format!("User with id {} not found", user_id)))
    }

    pub async fn get_by_email(&self, email: String) -> Result<User, ErrorBlog> {
        let user_repo = self.0.get_user_repo().await;
        user_repo
            .get_by_email(email.clone())
            .await?
            .ok_or_else(|| ErrorBlog::NotFound(format!("User with email {} not found", email)))
    }

    pub async fn update(
        &self,
        user_id: uuid::Uuid,
        username: Option<String>,
        email: Option<String>,
        password: Option<String>,
    ) -> Result<User, ErrorBlog> {
        if let (true, true, true) = (username.is_none(), email.is_none(), password.is_none()) {
            return Err(ErrorBlog::Validation("All field is none".to_string()));
        };

        let user_repo = self.0.get_user_repo().await;
        let mut user = user_repo
            .get_by_id(user_id)
            .await?
            .ok_or_else(|| ErrorBlog::NotFound(format!("User with id {} not found", user_id)))?;
        if let Some(email) = email {
            user.set_email(email)?;
        }
        if let Some(password) = password {
            user.set_password(password)?;
        }
        if let Some(username) = username {
            user.set_username(username);
        }
        user_repo.update(user_id, user.clone()).await
    }

    pub async fn delete(&self, user_id: uuid::Uuid) -> Result<User, ErrorBlog> {
        let user_repo = self.0.get_user_repo().await;
        user_repo.delete(user_id).await
    }
}
