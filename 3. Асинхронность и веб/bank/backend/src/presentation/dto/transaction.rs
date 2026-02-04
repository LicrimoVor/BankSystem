use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::transaction::Operation;

#[derive(Deserialize)]
pub struct WithdrawalDto {
    pub amount: f64,
}

#[derive(Deserialize)]
pub struct DepositDto {
    pub amount: f64,
}

#[derive(Deserialize)]
pub struct TransferDto {
    pub to_account_id: Uuid,
    pub amount: f64,
}

#[derive(Serialize)]
pub struct TransactionDto {
    pub id: Uuid,
    pub amount: f64,
    pub from: Option<Uuid>,
    pub to: Option<Uuid>,
    pub opeation: Operation,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<crate::domain::transaction::Transaction> for TransactionDto {
    fn from(transaction: crate::domain::transaction::Transaction) -> Self {
        Self {
            id: *transaction.id(),
            amount: *transaction.amount(),
            from: *transaction.from_id(),
            to: *transaction.to_id(),
            opeation: transaction.operation().clone(),
            created_at: *transaction.created_at(),
        }
    }
}
