use crate::Name;
use std::fmt::Display;

#[derive(Debug)]
pub enum BalanceManagerError {
    UserNotFound(Name),
    NotEnoughMoney { required: i64, available: i64 },
}

impl Display for BalanceManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BalanceManagerError::UserNotFound(name) => write!(f, "Пользователь {} не найден", name),
            BalanceManagerError::NotEnoughMoney {
                required,
                available,
            } => {
                write!(
                    f,
                    "Недостаточно средств. Необходимо: {}, Доступно: {}",
                    required, available
                )
            }
        }
    }
}

pub trait BalanceManager {
    fn deposit(&mut self, name: &Name, amount: i64) -> Result<(), BalanceManagerError>;
    fn withdraw(&mut self, name: &Name, amount: i64) -> Result<(), BalanceManagerError>;
}
