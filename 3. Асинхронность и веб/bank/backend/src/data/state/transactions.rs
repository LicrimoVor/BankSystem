use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    domain::{
        account::Account,
        transaction::{self, Transaction, TransactionRepository},
    },
    infrastructure::{error::ErrorApi, state::State},
};

pub struct TransactionStateRepo(pub Arc<State>);

impl TransactionRepository for TransactionStateRepo {
    async fn create_deposit(&mut self, amount: f64, to: &Account) -> Result<Transaction, ErrorApi> {
        let id = Uuid::new_v4();
        let created_at = Utc::now();
        let transaction = transaction::factory::create_deposit(id, amount, *to.id(), created_at);
        let mut transactions = self.0.transactions().await;
        let Some(trans) = transactions.get_mut(to.id()) else {
            return Err(ErrorApi::DataBase("Account not found".to_string()));
        };
        if trans.get(&id).is_some() {
            return Err(ErrorApi::DataBase(
                "Transaction id already exists".to_string(),
            ));
        };
        trans.insert(id, transaction.clone());

        Ok(transaction)
    }
    async fn create_withdrawal(
        &mut self,
        amount: f64,
        from: &Account,
    ) -> Result<Transaction, ErrorApi> {
        let id = Uuid::new_v4();
        let created_at = Utc::now();
        let transaction =
            transaction::factory::create_withdrawal(id, amount, *from.id(), created_at);
        let mut transactions = self.0.transactions().await;
        let Some(trans) = transactions.get_mut(from.id()) else {
            return Err(ErrorApi::DataBase("Account not found".to_string()));
        };
        if trans.get(&id).is_some() {
            return Err(ErrorApi::DataBase(
                "Transaction id already exists".to_string(),
            ));
        };
        trans.insert(id, transaction.clone());

        Ok(transaction)
    }
    async fn create_transfer(
        &mut self,
        amount: f64,
        from: &Account,
        to: &Account,
    ) -> Result<Transaction, ErrorApi> {
        let id = Uuid::new_v4();
        let created_at = Utc::now();
        let transaction =
            transaction::factory::create_transfer(id, amount, *from.id(), *to.id(), created_at);
        let mut transactions = self.0.transactions().await;
        let [Some(from_trans), Some(to_trans)] =
            transactions.get_disjoint_mut([from.id(), to.id()])
        else {
            return Err(ErrorApi::DataBase("Account not found".to_string()));
        };
        if from_trans.get(&id).is_some() {
            return Err(ErrorApi::DataBase(
                "Transaction id already exists".to_string(),
            ));
        };
        if to_trans.get(&id).is_some() {
            return Err(ErrorApi::DataBase(
                "Transaction id already exists".to_string(),
            ));
        };
        from_trans.insert(id, transaction.clone());
        to_trans.insert(id, transaction.clone());

        Ok(transaction)
    }
    async fn delete(&mut self, transaction: &Transaction) -> Result<(), ErrorApi> {
        let mut transactions = self.0.transactions().await;

        if let Some(from_id) = transaction.from() {
            let Some(trans) = transactions.get_mut(from_id) else {
                return Err(ErrorApi::DataBase("Account not found".to_string()));
            };
            if trans.remove(transaction.id()).is_none() {
                return Err(ErrorApi::DataBase("Transaction not found".to_string()));
            };
        }
        if let Some(to_id) = transaction.to() {
            let Some(trans) = transactions.get_mut(to_id) else {
                return Err(ErrorApi::DataBase("Account not found".to_string()));
            };
            if trans.remove(transaction.id()).is_none() {
                return Err(ErrorApi::DataBase("Transaction not found".to_string()));
            };
        }
        Ok(())
    }
    async fn get_by_id(&self, id: Uuid) -> Option<Transaction> {
        let transactions = self.0.transactions().await;
        let Some(res) = transactions.values().find(|trans| trans.get(&id).is_some()) else {
            return None;
        };
        res.get(&id).cloned()
    }
    async fn gets_by_account(&self, account: &Account) -> Option<Vec<Transaction>> {
        let transactions = self.0.transactions().await;
        let Some(trans) = transactions.get(account.id()) else {
            return None;
        };
        Some(trans.values().cloned().collect())
    }
}
