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

#[derive(Clone, PartialEq)]
pub enum BalanceOp {
    Deposit(u64),
    Withdraw(u64),
    Transfer(String, u64),
    Close,
}

impl Display for BalanceOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            BalanceOp::Deposit(v) => format!("Deposit({})", v),
            BalanceOp::Withdraw(v) => format!("Withdraw({})", v),
            BalanceOp::Transfer(n, v) => format!("Transfer({}, {})", n, v),
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
            BalanceOp::Transfer(n, v) => format!("T({},{})", n, v),
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
            BalanceOp::Transfer(n, v) => format!("T({}:{})", n, v),
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
        let val_len = val.len();
        if let Ok(v) = val.parse::<i64>() {
            return match op {
                "D" => Ok(BalanceOp::Deposit(v)),
                "W" => Ok(BalanceOp::Withdraw(v)),
                _ => Err(BalanceOpError::InvalidOperation(text)),
            };
        } else if let Some((name, v)) = val[1..val_len - 1].split_once(':') {
            return match op {
                "T" => Ok(BalanceOp::Transfer(
                    name.to_string(),
                    v.parse::<i64>()
                        .map_err(|_| BalanceOpError::ParseError(v.to_string()))?,
                )),
                _ => Err(BalanceOpError::InvalidOperation(text)),
            };
        }
        Err(BalanceOpError::ParseError(text))
    }
}

impl From<i64> for Balance {
    fn from(value: i64) -> Self {
        Balance::new(value, vec![])
    }
}
