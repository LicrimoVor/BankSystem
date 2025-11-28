use super::{
    BalanceSize,
    operations::{BalanceOp, Operation, OperationError, Status},
};
use std::fmt::Display;

/// Баланс
#[derive(Debug, Clone)]
pub struct Balance {
    value: BalanceSize,
    history: Vec<Operation>,
}

impl Display for Balance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let history = self
            .history
            .iter()
            .map(|op| format!("{:?}", op.tx_type))
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "{},[{}]", self.value, history)
    }
}

impl Default for Balance {
    fn default() -> Self {
        Self::new(0, vec![])
    }
}

impl From<i128> for Balance {
    fn from(value: i128) -> Self {
        Balance::new(value as BalanceSize, vec![])
    }
}

impl From<i64> for Balance {
    fn from(value: i64) -> Self {
        Balance::new(value as BalanceSize, vec![])
    }
}

impl Balance {
    pub fn new(value: BalanceSize, history: Vec<Operation>) -> Self {
        Balance { value, history }
    }

    /// Применяет операцию к счету
    pub fn apply_op(&mut self, op: Operation) -> Result<(), OperationError> {
        let result = match op.tx_type {
            BalanceOp::Deposit(b) => {
                if let Some(sum) = self.value.checked_add(b.into()) {
                    self.value = sum;
                    Ok(())
                } else {
                    Err(OperationError::OverLimitInt64)
                }
            }
            BalanceOp::Withdraw(b) => {
                if self.value < b.into() {
                    Err(OperationError::NotEnoughMoney {
                        required: b,
                        available: self.value,
                    })
                } else {
                    self.value -= b.into();
                    Ok(())
                }
            }
            BalanceOp::Transfer(_, b, f) => {
                if self.value < b {
                    Err(OperationError::NotEnoughMoney {
                        required: b,
                        available: self.value,
                    })
                } else {
                    self.value = b;
                    Ok(())
                }
            }
            BalanceOp::Close => {
                self.value = 0;
                Ok(())
            }
        };

        if result.is_ok() {
            op.set_status(Status::SUCCESS);
            self.history.push(op);
        } else {
            op.set_status(Status::FAILURE);
        }
        result
    }

    pub fn get_value(&self) -> BalanceSize {
        self.value
    }

    pub fn get_history(&self) -> &Vec<Operation> {
        &self.history
    }
}

impl TryFrom<String> for Balance {
    type Error = OperationError;
    fn try_from(text: String) -> Result<Self, Self::Error> {
        if text.is_empty() {
            return Err(OperationError::ParseError("Пустая строка".to_string()));
        }
        let Some((value, history)) = text.trim().split_once(',') else {
            return Err(OperationError::ParseError("Баланс некорректен".to_string()));
        };

        let value = value
            .parse::<i64>()
            .map_err(|_| OperationError::ParseError("Баланс некорректен".to_string()))?;

        let history_len = history.len();
        let history = history[1..history_len - 1] // убираем скобочки []
            .split(',')
            .map(|op| BalanceOp::try_from(op.to_string()))
            .collect::<Result<Vec<BalanceOp>, OperationError>>()?;
        Ok(Balance { value, history })
    }
}
