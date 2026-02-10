use crate::{
    domain::user::{User, UserRepository, factory},
    infrastructure::{errors::ErrorBlog, state::State},
};
use std::sync::Arc;
use uuid::Uuid;

pub struct UserStateRepo(pub Arc<State>);

#[async_trait::async_trait]
impl UserRepository for UserStateRepo {
    async fn create(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<User, ErrorBlog> {
        let user = factory::create(username, email, password)?;
        let mut user_state = self.0.get_mut_users().await;
        user_state.insert(user.id().clone(), user.clone());
        Ok(user)
    }

    async fn delete(&self, user_id: Uuid) -> Result<User, ErrorBlog> {
        let mut user_state = self.0.get_mut_users().await;
        if let Some(user) = user_state.remove(&user_id) {
            Ok(user)
        } else {
            Err(ErrorBlog::NotFound(format!(
                "User with id {} not found",
                user_id
            )))
        }
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, ErrorBlog> {
        let user_state = self.0.get_users().await;
        Ok(user_state.get(&id).cloned())
    }

    async fn get_by_email(&self, email: String) -> Result<Option<User>, ErrorBlog> {
        let user_state = self.0.get_users().await;
        Ok(user_state
            .values()
            .find(|user| user.email() == &email)
            .cloned())
    }

    async fn get_by_username(&self, username: String) -> Result<Option<User>, ErrorBlog> {
        let user_state = self.0.get_users().await;
        Ok(user_state
            .values()
            .find(|user| user.username() == &username)
            .cloned())
    }

    async fn update(&self, user_id: Uuid, user: User) -> Result<User, ErrorBlog> {
        let mut user_state = self.0.get_mut_users().await;
        if user_state.contains_key(&user_id) {
            user_state.insert(user_id, user.clone());
            Ok(user)
        } else {
            Err(ErrorBlog::NotFound(format!(
                "User with id {} not found",
                user_id
            )))
        }
    }
}
