use serde::Serialize;

use super::transaction::TransactionDto;

#[derive(Serialize)]
pub struct AccountDto {
    pub id: uuid::Uuid,
    pub balance: f64,
}

#[derive(Serialize)]
pub struct AccountsDto {
    pub accounts: Vec<AccountDto>,
}

#[derive(Serialize)]
pub struct AccountHistoryDto {
    pub history: Vec<TransactionDto>,
}
