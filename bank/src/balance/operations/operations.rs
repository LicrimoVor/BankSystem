use super::super::Balance;
use super::{OperationError, OperationStatus, OperationType};
use crate::Name;
use std::time::{SystemTime, UNIX_EPOCH};

/// Операция баланса
#[derive(Debug, Clone, PartialEq)]
pub struct Operation {
    id: u64,
    timestamp: u64,

    pub tx_type: OperationType,
    pub status: OperationStatus,
    pub description: String,
}

impl Operation {
    pub fn new(id: u64, tx_type: OperationType, description: Option<String>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Установите актуальное время")
            .as_secs();

        Self {
            id,
            tx_type,
            timestamp,
            status: OperationStatus::PENDING,
            description: description.unwrap_or(format!("Record number #{}", id)),
        }
    }

    /// Создает операцию депозита
    pub fn deposit(id: u64, amount: u64) -> Self {
        Self::new(id, OperationType::Deposit(amount), None)
    }

    /// Создает операцию снятия
    pub fn withdraw(id: u64, amount: u64) -> Self {
        Self::new(id, OperationType::Withdraw(amount), None)
    }

    /// Создает операцию перевода
    pub fn transfer(id: u64, name: Name, amount: u64, is_to: bool) -> Self {
        Self::new(id, OperationType::Transfer(name, amount, is_to), None)
    }

    /// Создает операцию закрытия
    pub fn close(id: u64) -> Self {
        Self::new(id, OperationType::Close, None)
    }

    /// Устанавливает статус операции
    pub fn set_status(&mut self, status: OperationStatus) {
        self.status = status;
    }

    /// Применяет операцию к счету
    pub fn apply(mut self, balance: &mut Balance) -> Result<(), OperationError> {
        if self.status != OperationStatus::PENDING {
            return Err(OperationError::InvalidStatus);
        }

        let result = match self.tx_type {
            OperationType::Deposit(b) => {
                if let Some(res) = balance.value.checked_add(b.into()) {
                    balance.value = res;
                    Ok(())
                } else {
                    Err(OperationError::OverLimitSize)
                }
            }
            OperationType::Withdraw(b) => {
                if let Some(res) = balance.value.checked_sub(b.into()) {
                    balance.value = res;
                    Ok(())
                } else {
                    Err(OperationError::NotEnoughMoney {
                        required: b,
                        available: balance.value,
                    })
                }
            }
            OperationType::Transfer(_, b, f) => {
                if !f {
                    if let Some(res) = balance.value.checked_sub(b.into()) {
                        balance.value = res;
                        Ok(())
                    } else {
                        Err(OperationError::NotEnoughMoney {
                            required: b,
                            available: balance.value,
                        })
                    }
                } else {
                    if let Some(res) = balance.value.checked_add(b.into()) {
                        balance.value = res;
                        Ok(())
                    } else {
                        Err(OperationError::OverLimitSize)
                    }
                }
            }
            OperationType::Close => {
                balance.value = 0;
                Ok(())
            }
        };

        if result.is_ok() {
            self.set_status(OperationStatus::SUCCESS);
        } else {
            self.set_status(OperationStatus::FAILURE);
        }
        balance.history.push(self);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balance_op_apply_deposit() {
        let mut balance = Balance::new(100, vec![]);
        let result = Operation::deposit(1, 50).apply(&mut balance);

        assert_eq!(result, Ok(()));
        assert_eq!(balance.value, 150);
        assert_eq!(balance.history.len(), 1);
        assert_eq!(
            balance.history.last().unwrap().status,
            OperationStatus::SUCCESS
        );
    }

    #[test]
    fn test_balance_op_apply_withdraw() {
        let mut balance = Balance::new(100, vec![]);
        let result = Operation::withdraw(1, 50).apply(&mut balance);

        assert_eq!(result, Ok(()));
        assert_eq!(balance.value, 50);
        assert_eq!(balance.history.len(), 1);
        assert_eq!(
            balance.history.last().unwrap().status,
            OperationStatus::SUCCESS
        );
    }

    #[test]
    fn test_balance_op_apply_transfer() {
        let mut balance_from = Balance::new(100, vec![]);
        let mut balance_to = Balance::new(25, vec![]);
        let result_from = Operation::transfer(1, "to".into(), 50, false).apply(&mut balance_from);
        let result_to = Operation::transfer(1, "from".into(), 50, true).apply(&mut balance_to);

        assert_eq!(result_from, Ok(()));
        assert_eq!(result_to, Ok(()));
        assert_eq!(balance_from.value, 50);
        assert_eq!(balance_to.value, 75);
        assert_eq!(balance_from.history.len(), 1);
        assert_eq!(balance_to.history.len(), 1);
        assert_eq!(
            balance_from.history.last().unwrap().status,
            OperationStatus::SUCCESS
        );
        assert_eq!(
            balance_to.history.last().unwrap().status,
            OperationStatus::SUCCESS
        );
    }

    #[test]
    fn test_balance_op_apply_close() {
        let mut balance = Balance::new(100, vec![]);
        let result = Operation::close(1).apply(&mut balance);

        assert_eq!(result, Ok(()));
        assert_eq!(balance.value, 0);
        assert_eq!(balance.history.len(), 1);
        assert_eq!(
            balance.history.last().unwrap().status,
            OperationStatus::SUCCESS
        );
    }

    #[test]
    fn test_balance_op_apply_invalid_status() {
        let mut balance = Balance::new(100, vec![]);
        let mut op = Operation::withdraw(1, 150);
        op.set_status(OperationStatus::SUCCESS);
        let result = Operation::withdraw(1, 150).apply(&mut balance);

        assert_eq!(result, Err(OperationError::InvalidStatus));
        assert_eq!(balance.value, 100);
        assert_eq!(balance.history.len(), 1);
        assert_eq!(
            balance.history.last().unwrap().status,
            OperationStatus::FAILURE
        );
    }

    #[test]
    fn test_balance_op_apply_invalid_money() {
        let mut balance = Balance::new(100, vec![]);
        let result = Operation::withdraw(1, 150).apply(&mut balance);

        assert_eq!(result, Err(OperationError::InvalidStatus));
        assert_eq!(balance.value, 100);
        assert_eq!(balance.history.len(), 1);
        assert_eq!(
            balance.history.last().unwrap().status,
            OperationStatus::FAILURE
        );
    }

    #[test]
    fn test_balance_op_set_status() {
        let mut op = Operation::withdraw(1, 150);
        op.set_status(OperationStatus::SUCCESS);
        assert_eq!(op.status, OperationStatus::SUCCESS);

        op.set_status(OperationStatus::FAILURE);
        assert_eq!(op.status, OperationStatus::FAILURE);

        op.set_status(OperationStatus::PENDING);
        assert_eq!(op.status, OperationStatus::PENDING);
    }
}
