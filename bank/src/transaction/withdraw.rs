use super::{Transaction, TxError};
use crate::balance::{
    manager::{BalanceManager, BalanceManagerError},
    operations::{OperationAmount, OperationType},
};
use crate::storage::Storage;

#[derive(Debug, Clone)]
pub struct Withdraw {
    account: String,
    amount: OperationAmount,
}

/// Списание с счета
impl Withdraw {
    pub fn new(account: String, amount: OperationAmount) -> Self {
        Self { account, amount }
    }
}

impl Transaction for Withdraw {
    fn apply(&self, storage: &mut Storage) -> Result<(), TxError> {
        storage
            .withdraw(&self.account, self.amount)
            .map_err(|e| match e {
                BalanceManagerError::OperationError(err) => TxError::OperationError(err),
                BalanceManagerError::UserNotFound(_) => TxError::InvalidAccount,
            })?;
        Ok(())
    }
}

impl From<Withdraw> for OperationType {
    fn from(val: Withdraw) -> Self {
        OperationType::Withdraw(val.amount)
    }
}
