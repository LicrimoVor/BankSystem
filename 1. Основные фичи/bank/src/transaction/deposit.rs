use super::{Transaction, TxError};
use crate::balance::{
    manager::BalanceManager,
    operations::{OperationAmount, OperationType},
};
use crate::storage::Storage;

/// Пополнение счета
#[derive(Debug, Clone)]
pub struct Deposit {
    account: String,
    amount: OperationAmount,
}

impl Deposit {
    pub fn new(account: String, amount: OperationAmount) -> Self {
        Self { account, amount }
    }
}

impl Transaction for Deposit {
    fn apply(&self, storage: &mut Storage) -> Result<(), TxError> {
        storage
            .deposit(&self.account, self.amount)
            .map_err(|_| TxError::InvalidAccount)?;
        Ok(())
    }
}

impl From<Deposit> for OperationType {
    fn from(val: Deposit) -> Self {
        OperationType::Deposit(val.amount)
    }
}
