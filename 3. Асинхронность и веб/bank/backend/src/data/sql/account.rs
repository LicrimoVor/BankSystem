use crate::{
    domain::{
        account::{Account, AccountRepository},
        user::User,
    },
    infrastructure::error::ErrorApi,
};
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct AccountSQLRepo(pub Arc<PgPool>);

#[async_trait]
impl AccountRepository for AccountSQLRepo {
    async fn create(
        &mut self,
        user: &User,
        init_balance: Option<f64>,
    ) -> Result<Account, ErrorApi> {
        todo!()
    }

    async fn update(&mut self, account: &Account) -> Result<(), ErrorApi> {
        todo!()
    }
    async fn delete(&mut self, account: &Account) -> Result<(), ErrorApi> {
        todo!()
    }
    async fn get_by_id(&self, id: Uuid) -> Option<Account> {
        todo!()
    }
    async fn gets_by_user(&self, user: &User) -> Option<Vec<Account>> {
        todo!()
    }
}
