use async_trait::async_trait;
use getset::Getters;
use serde::Serialize;
use uuid::Uuid;

use crate::{domain::account::Account, impl_constructor, infrastructure::error::ErrorApi};

#[derive(Debug, Serialize, Clone, sqlx::Type)]
#[sqlx(type_name = "operation", rename_all = "lowercase")]
pub enum Operation {
    DEPOSIT,
    WITHDRAWAL,
    TRANSFER,
}

#[derive(Debug, Serialize, Getters, Clone)]
pub struct Transaction {
    #[getset(get = "pub")]
    id: Uuid,

    #[getset(get = "pub")]
    operation: Operation,

    #[getset(get = "pub")]
    amount: f64,

    /// uuid account
    #[getset(get = "pub")]
    from_id: Option<Uuid>,

    /// uuid account
    #[getset(get = "pub")]
    to_id: Option<Uuid>,

    #[getset(get = "pub")]
    created_at: chrono::DateTime<chrono::Utc>,
}

#[async_trait]
pub trait TransactionRepository: Send + Sync {
    async fn create_deposit(&mut self, amount: f64, to: &Account) -> Result<Transaction, ErrorApi>;
    async fn create_withdrawal(
        &mut self,
        amount: f64,
        from: &Account,
    ) -> Result<Transaction, ErrorApi>;
    async fn create_transfer(
        &mut self,
        amount: f64,
        from: &Account,
        to: &Account,
    ) -> Result<Transaction, ErrorApi>;
    async fn delete(&mut self, transaction: &Transaction) -> Result<(), ErrorApi>;
    async fn get_by_id(&self, id: Uuid) -> Option<Transaction>;
    async fn gets_by_account(&self, account: &Account) -> Option<Vec<Transaction>>;
}
impl_constructor!(token: TransactionToken, Transaction, (
    id: Uuid,
    operation: Operation,
    amount: f64,
    from_id: Option<Uuid>,
    to_id: Option<Uuid>,
    created_at: chrono::DateTime<chrono::Utc>
));

pub mod factory {
    use super::*;

    pub fn create_deposit(
        id: Uuid,
        amount: f64,
        to: Uuid,
        created_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<Transaction, ErrorApi> {
        if amount < 0.0 {
            return Err(ErrorApi::Validation(
                "Deposit amount cannot be negative".to_string(),
            ));
        }

        Ok(Transaction {
            id,
            operation: Operation::DEPOSIT,
            amount,
            from_id: None,
            to_id: Some(to),
            created_at,
        })
    }

    pub fn create_withdrawal(
        id: Uuid,
        amount: f64,
        from: Uuid,
        created_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<Transaction, ErrorApi> {
        if amount < 0.0 {
            return Err(ErrorApi::Validation(
                "Deposit amount cannot be negative".to_string(),
            ));
        }
        Ok(Transaction {
            id,
            operation: Operation::WITHDRAWAL,
            amount,
            from_id: Some(from),
            to_id: None,
            created_at,
        })
    }

    pub fn create_transfer(
        id: Uuid,
        amount: f64,
        from: Uuid,
        to: Uuid,
        created_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<Transaction, ErrorApi> {
        if amount < 0.0 {
            return Err(ErrorApi::Validation(
                "Deposit amount cannot be negative".to_string(),
            ));
        }

        if to == from {
            return Err(ErrorApi::Validation(
                "Transfer to and from cannot be the same".to_string(),
            ));
        }

        Ok(Transaction {
            id,
            operation: Operation::TRANSFER,
            amount,
            from_id: Some(from),
            to_id: Some(to),
            created_at,
        })
    }
}
