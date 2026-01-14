mod combine;
mod deposit;
mod error;
mod transfer;
mod withdraw;

pub use combine::TxCombinator;
pub use deposit::Deposit;
pub use error::TxError;
pub use transfer::Transfer;
pub use withdraw::Withdraw;
pub mod macros;

use crate::{impl_add_trait, storage::Storage};

/// Транзакция - трейт для всех транзакций, которые можно применить к балансу
/// - ```fn apply(&self, storage: &mut Storage)``` - применить транзакцию
pub trait Transaction {
    fn apply(&self, storage: &mut Storage) -> Result<(), TxError>;
}

impl_add_trait!(Deposit, Withdraw, Transfer);
