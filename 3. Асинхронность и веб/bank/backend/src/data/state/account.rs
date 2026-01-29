use std::sync::Arc;

use uuid::Uuid;

use crate::{
    domain::{
        account::{self, Account, AccountRepository},
        user::User,
    },
    infrastructure::{error::ErrorApi, state::State},
};

pub struct AccountStateRepo(pub Arc<State>);

impl AccountRepository for AccountStateRepo {
    async fn create(
        &mut self,
        user: &User,
        init_balance: Option<f64>,
    ) -> Result<Account, ErrorApi> {
        let id = Uuid::new_v4();
        let mut accounts = self.0.accounts().await;
        let account = account::factory::create(id, *user.id(), init_balance.unwrap_or(0.0));
        let Some(accs) = accounts.get_mut(&user.id()) else {
            return Err(ErrorApi::DataBase("User not found".to_string()));
        };
        accs.insert(id, account.clone());

        Ok(account)
    }

    async fn update(&mut self, account: &Account) -> Result<(), ErrorApi> {
        let mut accounts = self.0.accounts().await;
        let Some(accs) = accounts.get_mut(&account.user_id()) else {
            return Err(ErrorApi::DataBase("Account not found".to_string()));
        };
        let Some(acc) = accs.get_mut(&account.id()) else {
            return Err(ErrorApi::DataBase("Account not found".to_string()));
        };
        *acc = account.clone();
        Ok(())
    }
    async fn delete(&mut self, account: &Account) -> Result<(), ErrorApi> {
        let mut accounts = self.0.accounts().await;
        let Some(accs) = accounts.get_mut(&account.user_id()) else {
            return Err(ErrorApi::DataBase("Account not found".to_string()));
        };
        accs.remove(&account.id());
        Ok(())
    }
    async fn get_by_id(&self, id: Uuid) -> Option<Account> {
        let accounts = self.0.accounts().await;
        let Some(res) = accounts.values().find(|accs| {
            return accs.get(&id).is_some();
        }) else {
            return None;
        };
        Some(res.get(&id).unwrap().clone())
    }
    async fn gets_by_user(&self, user: &User) -> Option<Vec<Account>> {
        let accounts = self.0.accounts().await;
        let Some(accs) = accounts.get(&user.id()) else {
            return None;
        };
        Some(accs.values().map(|acc| acc.clone()).collect())
    }
}
