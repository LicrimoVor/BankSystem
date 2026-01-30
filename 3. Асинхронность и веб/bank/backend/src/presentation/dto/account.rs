use serde::Serialize;

use super::transaction::TransactionDto;

#[derive(Serialize)]
pub struct AccountDto {
    pub id: uuid::Uuid,
    pub balance: f64,
}

impl From<crate::domain::account::Account> for AccountDto {
    fn from(account: crate::domain::account::Account) -> Self {
        Self {
            id: *account.id(),
            balance: *account.balance(),
        }
    }
}

#[derive(Serialize)]
pub struct AccountsDto {
    pub accounts: Vec<AccountDto>,
}

#[derive(Serialize)]
pub struct AccountHistoryDto {
    pub history: Vec<TransactionDto>,
}
