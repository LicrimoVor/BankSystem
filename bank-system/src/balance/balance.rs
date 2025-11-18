use super::{
    Balance,
    operations::{BalanceOp, BalanceOpError},
};
use std::fmt::Display;

impl Display for Balance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let history = self
            .history
            .iter()
            .map(|op| format!("{:?}", op))
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "{},[{}]", self.value, history)
    }
}

impl Balance {
    pub fn new(value: i64) -> Self {
        Balance {
            value,
            history: vec![],
        }
    }

    pub fn apply_op(&mut self, op: &BalanceOp) -> Result<(), BalanceOpError> {
        let result = match op {
            BalanceOp::Deposit(b) => {
                if let Some(sum) = self.value.checked_add(*b) {
                    self.value = sum;
                    Ok(())
                } else {
                    Err(BalanceOpError::OverLimitInt64)
                }
            }
            BalanceOp::Withdraw(b) => {
                if self.value < *b {
                    Err(BalanceOpError::NotEnoughMoney {
                        required: *b,
                        available: self.value,
                    })
                } else {
                    self.value -= b;
                    Ok(())
                }
            }
            BalanceOp::Close => {
                self.value = 0;
                Ok(())
            }
        };

        if result.is_ok() {
            self.history.push(*op);
        }
        result
    }

    pub fn proccess<'a>(&mut self, ops: &[&'a BalanceOp]) -> Vec<&'a BalanceOp> {
        let exclusion = ops
            .into_iter()
            .filter(|op: &&&'a BalanceOp| self.apply_op(*op).is_ok())
            .map(|op| *op)
            .collect();
        exclusion
    }

    pub fn get_value(&self) -> i64 {
        self.value
    }

    pub fn get_history(&self) -> &Vec<BalanceOp> {
        &self.history
    }
}

impl TryFrom<String> for Balance {
    type Error = BalanceOpError;
    fn try_from(text: String) -> Result<Self, Self::Error> {
        if text.is_empty() {
            return Err(BalanceOpError::ParseError("Пустая строка".to_string()));
        }
        let Some((value, history)) = text.trim().split_once(',') else {
            return Err(BalanceOpError::ParseError("Баланс некорректен".to_string()));
        };

        let value = value
            .parse::<i64>()
            .map_err(|_| BalanceOpError::ParseError("Баланс некорректен".to_string()))?;

        let history_len = history.len();
        let history = history[1..history_len - 1] // убираем скобочки []
            .split(',')
            .map(|op| BalanceOp::try_from(op.to_string()))
            .collect::<Result<Vec<BalanceOp>, BalanceOpError>>()?;
        Ok(Balance { value, history })
    }
}
