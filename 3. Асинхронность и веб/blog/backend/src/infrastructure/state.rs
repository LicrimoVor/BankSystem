use crate::domain::{auth::RefreshToken, post::Post, user::User};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use uuid::Uuid;

/// Модуль, отвечающий за состояние приложения
/// в случае использования in-memory БД
#[derive(Debug, Clone)]
pub struct State {
    /// Хранилище пользователей
    /// {user_id: User}
    users: Arc<RwLock<HashMap<Uuid, User>>>,
    /// Хранилище постов
    /// {user_id: {post_id: post}}
    posts: Arc<RwLock<HashMap<Uuid, HashMap<Uuid, Post>>>>,
    /// Хранилище refresh токенов
    /// {refresh_token: user_id}
    refresh_tokens: Arc<RwLock<HashMap<RefreshToken, Uuid>>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
            posts: Arc::new(RwLock::new(HashMap::new())),
            refresh_tokens: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get_users(&self) -> tokio::sync::RwLockReadGuard<'_, HashMap<Uuid, User>> {
        self.users.read().await
    }

    pub async fn get_refresh_tokens(
        &self,
    ) -> tokio::sync::RwLockReadGuard<'_, HashMap<RefreshToken, Uuid>> {
        self.refresh_tokens.read().await
    }

    pub async fn get_posts(
        &self,
    ) -> tokio::sync::RwLockReadGuard<'_, HashMap<Uuid, HashMap<Uuid, Post>>> {
        self.posts.read().await
    }

    pub async fn get_mut_users(&self) -> tokio::sync::RwLockWriteGuard<'_, HashMap<Uuid, User>> {
        self.users.write().await
    }

    pub async fn get_mut_posts(
        &self,
    ) -> tokio::sync::RwLockWriteGuard<'_, HashMap<Uuid, HashMap<Uuid, Post>>> {
        self.posts.write().await
    }

    pub async fn get_mut_refresh_tokens(
        &self,
    ) -> tokio::sync::RwLockWriteGuard<'_, HashMap<RefreshToken, Uuid>> {
        self.refresh_tokens.write().await
    }
}
