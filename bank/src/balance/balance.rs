use super::{BalanceSize, operations::Operation};
use std::fmt::Display;

/// Баланс
#[derive(Debug, Clone, PartialEq)]
pub struct Balance {
    pub(super) value: BalanceSize,
    pub(super) history: Vec<Operation>,
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

impl From<i32> for Balance {
    fn from(value: i32) -> Self {
        Balance::new(value as BalanceSize, vec![])
    }
}

impl Balance {
    pub fn new(value: BalanceSize, history: Vec<Operation>) -> Self {
        Balance { value, history }
    }

    pub fn get_value(&self) -> BalanceSize {
        self.value
    }

    pub fn get_history(&self) -> &Vec<Operation> {
        &self.history
    }
}
