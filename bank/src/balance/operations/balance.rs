use super::super::{BalanceSize, OperationError};
use std::fmt::{Debug, Display};

#[derive(Clone, PartialEq)]
pub enum BalanceOp {
    Deposit(u64),
    Withdraw(u64),
    Transfer(String, u64, bool),
    Close,
}

impl BalanceOp {
    fn get_amount(&self) -> BalanceSize {
        match self {
            BalanceOp::Transfer(_, v, f) => {
                if *f {
                    (*v).into()
                } else {
                    -(<u64 as Into<BalanceSize>>::into(*v))
                }
            }
            BalanceOp::Withdraw(v) => (*v).into(),
            BalanceOp::Deposit(v) => (*v).into(),
            BalanceOp::Close => 0,
        }
    }
}

impl Display for BalanceOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            BalanceOp::Deposit(v) => format!("Deposit({})", v),
            BalanceOp::Withdraw(v) => format!("Withdraw({})", v),
            BalanceOp::Transfer(n, v, f) => format!("Transfer({}, {}, {})", n, v, f),
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
            BalanceOp::Transfer(n, v, f) => format!("T({},{},{})", n, v, f),
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
            BalanceOp::Transfer(n, v, f) => format!("T({}:{}:{})", n, v, f),
            BalanceOp::Close => "C".to_string(),
        }
    }
}

impl TryFrom<String> for BalanceOp {
    type Error = OperationError;

    fn try_from(text: String) -> Result<Self, Self::Error> {
        if text.len() < 2 && text != "C" {
            return Err(OperationError::ParseError(text));
        }
        if text.len() == 1 && text == "C" {
            return Ok(BalanceOp::Close);
        }

        let (op, val) = text.split_at(1);
        let val_len = val.len();
        if let Ok(v) = val.parse::<u64>() {
            return match op {
                "D" => Ok(BalanceOp::Deposit(v)),
                "W" => Ok(BalanceOp::Withdraw(v)),
                _ => Err(OperationError::InvalidOperation(text)),
            };
        } else if let Some((name, val_flag)) = val[1..val_len - 1].split_once(':') {
            let Some((val, flag)) = val_flag.split_once(':') else {
                return Err(OperationError::ParseError(text));
            };
            let val = val
                .parse::<u64>()
                .map_err(|_| OperationError::ParseError(val.to_string()))?;
            if !(flag == "1" && flag == "0") {
                return Err(OperationError::ParseError(text));
            }
            let flag = flag == "1";
            return match op {
                "T" => Ok(BalanceOp::Transfer(name.to_string(), val, flag)),
                _ => Err(OperationError::InvalidOperation(text)),
            };
        }
        Err(OperationError::ParseError(text))
    }
}
