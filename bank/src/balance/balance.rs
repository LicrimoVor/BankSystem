use super::{BalanceSize, errors::BalanceError, operations::Operation};
use std::fmt::Display;

/// # Баланс
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

impl TryFrom<String> for Balance {
    type Error = BalanceError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let (value, history) = value
            .split_once(',')
            .ok_or(BalanceError::InvalidParseBalance(
                "Нет баланса.".to_string(),
            ))?;

        let value = value
            .parse::<BalanceSize>()
            .map_err(|_| BalanceError::InvalidParseBalance(value.to_string()))?;

        let len_history = history.len();
        let history = history[1..len_history - 1]
            .split('|')
            .map(|op| {
                Operation::try_from(op.to_string())
                    .map_err(|err| BalanceError::InvalidParseOperation(err))
            })
            .collect::<Result<Vec<Operation>, BalanceError>>()?;
        Ok(Balance { value, history })
    }
}

impl Balance {
    /// Создание баланса
    pub fn new(value: BalanceSize, history: Vec<Operation>) -> Self {
        Balance { value, history }
    }

    /// Получение баланса
    pub fn get_value(&self) -> BalanceSize {
        self.value
    }

    /// Получение истории
    pub fn get_history(&self) -> &Vec<Operation> {
        &self.history
    }

    pub(crate) fn save(&self) -> String {
        let history = self
            .history
            .iter()
            .map(|op| op.into())
            .collect::<Vec<String>>()
            .join("|");
        format!("{},[{}]", self.value, history)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balance_try_from() {
        let balance = "100,[1,1764444526,D100,success,Record number #1|3,1764444535,T(Julia:200:true),success,Record number #3]".to_string();
        let balance = Balance::try_from(balance);
        assert!(balance.is_ok());
    }

    #[test]
    fn test_balance_try_from_error() {
        let balance = "100,[1,1764444526,D100,success,Record number #1|3,1764444535,O(Julia:200:true),success,Record number #3".to_string();
        let balance = Balance::try_from(balance);
        assert!(balance.is_err());
    }

    #[test]
    fn test_balance_load_save() {
        let balance = Balance::try_from("100,[1,1764444526,D100,success,Record number #1|3,1764444535,T(Julia:200:true),success,Record number #3]".to_string());
        assert!(balance.is_ok());

        let balance = balance.unwrap();

        assert_eq!(
            balance.save(),
            "100,[1,1764444526,D100,success,Record number #1|3,1764444535,T(Julia:200:true),success,Record number #3]"
        );
    }
}
