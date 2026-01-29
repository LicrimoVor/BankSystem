use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    domain::user::{self, User, UserRepository},
    infrastructure::{error::ErrorApi, state::State},
};

pub struct UserStateRepo(pub Arc<State>);

impl UserRepository for UserStateRepo {
    async fn create(&mut self, email: String, password_hash: String) -> Result<User, ErrorApi> {
        let id = Uuid::new_v4();
        let created_at = Utc::now();
        let user = user::factory::create(id, created_at, email, password_hash);
        let mut users = self.0.users().await;
        if users.get(&id).is_some() {
            return Err(ErrorApi::DataBase("User id already exists".to_string()));
        }
        users.insert(id, user.clone());
        return Ok(user);
    }
    async fn update(&mut self, user: &User) -> Result<(), ErrorApi> {
        let mut users = self.0.users().await;
        if users.get(&user.id()).is_none() {
            return Err(ErrorApi::DataBase("User not found".to_string()));
        }
        users.insert(user.id().clone(), user.clone());
        return Ok(());
    }
    async fn delete(&mut self, user: &User) -> Result<(), ErrorApi> {
        let mut users = self.0.users().await;
        if users.remove(user.id()).is_none() {
            return Err(ErrorApi::DataBase("User not found".to_string()));
        };
        Ok(())
    }
    async fn get_by_email(&self, email: String) -> Option<User> {
        let users = self.0.users().await;
        users.values().find(|user| **user.email() == email).cloned()
    }
    async fn get_by_id(&self, id: Uuid) -> Option<User> {
        let users = self.0.users().await;
        users.get(&id).cloned()
    }
}
