mod combine;
mod deposit;
mod transfer;
mod withdraw;

pub use combine::TxCombinator;
pub use deposit::Deposit;
pub use transfer::Transfer;
pub use withdraw::Withdraw;
pub mod macros;

use crate::{Storage, impl_add_trait};

/// # Ошибки
/// - [TxError::InsufficientFunds] - Недостаточно средств
/// - [TxError::InvalidAccount] - Не найден счет
#[derive(Debug, PartialEq)]
pub enum TxError {
    InsufficientFunds,
    InvalidAccount,
}

/// Транзакция - трейт для всех транзакций, которые можно применить к балансу
/// - ```fn apply(&self, storage: &mut Storage)``` - применить транзакцию
pub trait Transaction {
    fn apply(&self, storage: &mut Storage) -> Result<(), TxError>;
}

impl_add_trait!(Deposit, Withdraw, Transfer);
