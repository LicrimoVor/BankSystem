use serde::Serialize;

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
