use super::{Transaction, TxError};
use crate::balance::{
    manager::{BalanceManager, BalanceManagerError},
    operations::{OperationAmount, OperationType},
};
use crate::storage::Storage;

#[derive(Debug, Clone)]
pub struct Transfer {
    from: String,
    to: String,
    amount: OperationAmount,
}

/// Перевод средств между счетами
impl Transfer {
    pub fn new(from: String, to: String, amount: OperationAmount) -> Self {
        Self { from, to, amount }
    }
}

impl Transaction for Transfer {
    fn apply(&self, storage: &mut Storage) -> Result<(), TxError> {
        storage
            .transfer(&self.from, &self.to, self.amount)
            .map_err(|e| match e {
                BalanceManagerError::OperationError { .. } => TxError::InsufficientFunds,
                BalanceManagerError::UserNotFound(_) => TxError::InvalidAccount,
            })?;

        Ok(())
    }
}

impl Into<OperationType> for Transfer {
    fn into(self) -> OperationType {
        OperationType::Withdraw(self.amount)
    }
}
