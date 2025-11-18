use super::Balance;
use std::{
    fmt::{Debug, Display},
    ops::{AddAssign, SubAssign},
};

pub enum BalanceOpError {
    NotEnoughMoney { required: i64, available: i64 },
    InvalidOperation(String),
    ParseError(String),
    OverLimitInt64,
}

#[derive(Clone, Copy, PartialEq)]
pub enum BalanceOp {
    Deposit(i64),
    Withdraw(i64),
    Close,
}

impl Display for BalanceOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            BalanceOp::Deposit(v) => format!("Deposit({})", v),
            BalanceOp::Withdraw(v) => format!("Withdraw({})", v),
            BalanceOp::Close => "Close".to_string(),
        };
        write!(f, "{label}")
    }
}

impl Debug for BalanceOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            BalanceOp::Deposit(v) => format!("D{}", v),
            BalanceOp::Withdraw(v) => format!("W{}", v),
            BalanceOp::Close => "C".to_string(),
        };
        write!(f, "{label}")
    }
}

impl Into<String> for BalanceOp {
    fn into(self) -> String {
        match self {
            BalanceOp::Deposit(v) => format!("D{}", v),
            BalanceOp::Withdraw(v) => format!("W{}", v),
            BalanceOp::Close => "C".to_string(),
        }
    }
}

impl TryFrom<String> for BalanceOp {
    type Error = BalanceOpError;

    fn try_from(text: String) -> Result<Self, Self::Error> {
        if text.len() < 2 && text != "C" {
            return Err(BalanceOpError::ParseError(text));
        }
        if text.len() == 1 && text == "C" {
            return Ok(BalanceOp::Close);
        }

        let (op, val) = text.split_at(1);
        if let Ok(v) = val.parse::<i64>() {
            return match op {
                "D" => Ok(BalanceOp::Deposit(v)),
                "W" => Ok(BalanceOp::Withdraw(v)),
                _ => Err(BalanceOpError::InvalidOperation(text)),
            };
        }
        Err(BalanceOpError::ParseError(text))
    }
}

impl From<i64> for Balance {
    fn from(value: i64) -> Self {
        Balance {
            value,
            history: vec![],
        }
    }
}

impl AddAssign<i64> for Balance {
    fn add_assign(&mut self, rhs: i64) {
        let _ = self.apply_op(&BalanceOp::Deposit(rhs));
    }
}

impl SubAssign<i64> for Balance {
    fn sub_assign(&mut self, rhs: i64) {
        let _ = self.apply_op(&BalanceOp::Withdraw(rhs));
    }
}
