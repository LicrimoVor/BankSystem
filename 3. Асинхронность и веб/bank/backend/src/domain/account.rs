use getset::{Getters, Setters};
use serde::Serialize;
use uuid::Uuid;

use crate::{domain::user::User, impl_constructor, infrastructure::error::ErrorApi};

#[derive(Debug, Serialize, Getters, Setters, Clone)]
pub struct Account {
    /// Account id
    #[getset(get = "pub")]
    id: Uuid,

    /// User id
    #[getset(get = "pub")]
    user_id: Uuid,

    /// Account balance
    #[getset(get = "pub", set = "pub")]
    balance: f64,
}

pub trait AccountRepository {
    async fn create(&mut self, user: &User, init_balance: Option<f64>)
        -> Result<Account, ErrorApi>;
    async fn update(&mut self, account: &Account) -> Result<(), ErrorApi>;
    async fn delete(&mut self, account: &Account) -> Result<(), ErrorApi>;
    async fn get_by_id(&self, id: Uuid) -> Option<Account>;
    async fn gets_by_user(&self, user: &User) -> Option<Vec<Account>>;
}

impl_constructor!(token: AccountToken, Account, (id: Uuid, user_id: Uuid, balance: f64));
impl_constructor!(factory: Account, (id: Uuid, user_id: Uuid, balance: f64));
