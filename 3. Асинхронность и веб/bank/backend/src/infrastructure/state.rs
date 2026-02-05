use crate::domain::{
    account::Account, course::Course, token::RefreshToken, transaction::Transaction, user::User,
};
use chrono::{DateTime, Utc};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{Mutex, MutexGuard};

#[derive(Debug, Clone)]
pub struct State {
    /// HashMap<user_id, user>
    users: Arc<Mutex<HashMap<uuid::Uuid, User>>>,
    /// HashMap<user_id, HashMap<account_id, account>>
    accounts: Arc<Mutex<HashMap<uuid::Uuid, HashMap<uuid::Uuid, Account>>>>,
    /// HashMap<account_id, HashMap<transaction_id, transaction>>
    transactions: Arc<Mutex<HashMap<uuid::Uuid, HashMap<uuid::Uuid, Transaction>>>>,
    /// HashMap<refresh_token_hash, user_id>
    refresh_tokens: Arc<Mutex<HashMap<String, RefreshToken>>>,
    /// HashMap<time_update_utc, course>
    courses: Arc<Mutex<HashMap<DateTime<Utc>, Course>>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(HashMap::new())),
            accounts: Arc::new(Mutex::new(HashMap::new())),
            transactions: Arc::new(Mutex::new(HashMap::new())),
            refresh_tokens: Arc::new(Mutex::new(HashMap::new())),
            courses: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn users(&self) -> MutexGuard<'_, HashMap<uuid::Uuid, User>> {
        self.users.lock().await
    }

    pub async fn accounts(
        &self,
    ) -> MutexGuard<'_, HashMap<uuid::Uuid, HashMap<uuid::Uuid, Account>>> {
        self.accounts.lock().await
    }

    pub async fn transactions(
        &self,
    ) -> MutexGuard<'_, HashMap<uuid::Uuid, HashMap<uuid::Uuid, Transaction>>> {
        self.transactions.lock().await
    }

    pub async fn refresh_tokens(&self) -> MutexGuard<'_, HashMap<String, RefreshToken>> {
        self.refresh_tokens.lock().await
    }

    pub async fn courses(&self) -> MutexGuard<'_, HashMap<DateTime<Utc>, Course>> {
        self.courses.lock().await
    }
}
