use std::{collections::HashMap, sync::Arc};

use tokio::sync::{Mutex, MutexGuard};

use crate::domain::{account::Account, user::User};

pub struct State {
    users: Arc<Mutex<HashMap<uuid::Uuid, User>>>,
    accounts: Arc<Mutex<HashMap<uuid::Uuid, HashMap<uuid::Uuid, Account>>>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(HashMap::new())),
            accounts: Arc::new(Mutex::new(HashMap::new())),
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
}
