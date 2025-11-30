use super::OperationError;
use std::fmt::{Debug, Display};

pub type OperationAmount = u64;

#[derive(Clone, PartialEq)]
pub enum OperationType {
    Deposit(OperationAmount),
    Withdraw(OperationAmount),
    /// name, amount, is_to
    Transfer(String, OperationAmount, bool),
    Close,
}

impl Display for OperationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            OperationType::Deposit(v) => format!("Deposit({})", v),
            OperationType::Withdraw(v) => format!("Withdraw({})", v),
            OperationType::Transfer(n, v, f) => format!("Transfer({}, {}, {})", n, v, f),
            OperationType::Close => "Close".to_string(),
        };
        write!(f, "{label}")
    }
}

impl Debug for OperationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            OperationType::Deposit(v) => format!("D{}", v),
            OperationType::Withdraw(v) => format!("W{}", v),
            OperationType::Transfer(n, v, f) => format!("T({}:{}:{})", n, v, f),
            OperationType::Close => "C".to_string(),
        };
        write!(f, "{label}")
    }
}

impl Into<String> for OperationType {
    fn into(self) -> String {
        match self {
            OperationType::Deposit(v) => format!("D{}", v),
            OperationType::Withdraw(v) => format!("W{}", v),
            OperationType::Transfer(n, v, f) => format!("T({}:{}:{})", n, v, f),
            OperationType::Close => "C".to_string(),
        }
    }
}

impl TryFrom<String> for OperationType {
    type Error = OperationError;

    fn try_from(text: String) -> Result<Self, Self::Error> {
        if text.len() < 2 && text != "C" {
            return Err(OperationError::ParseError(text));
        }
        if text.len() == 1 && text == "C" {
            return Ok(OperationType::Close);
        }

        let (op, val) = text.split_at(1);
        let val_len = val.len();
        if let Ok(v) = val.parse::<u64>() {
            return match op {
                "D" => Ok(OperationType::Deposit(v)),
                "W" => Ok(OperationType::Withdraw(v)),
                _ => Err(OperationError::InvalidOperation(text)),
            };
        } else {
            let parts: Vec<&str> = val[1..val_len - 1].splitn(3, ':').collect();
            if let [name, value, flag] = parts.as_slice() {
                let value = value
                    .parse::<u64>()
                    .map_err(|_| OperationError::ParseError(val.to_string()))?;
                if !(*flag == "false" || *flag == "true") {
                    return Err(OperationError::ParseError(text));
                }
                let flag = *flag == "true";
                return match op {
                    "T" => Ok(OperationType::Transfer(name.to_string(), value, flag)),
                    _ => Err(OperationError::InvalidOperation(text)),
                };
            }
            Err(OperationError::ParseError(text))
        }
    }
}
