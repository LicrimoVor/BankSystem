#![deny(unreachable_pub)]
pub mod errors;
pub mod from;
pub mod to;
pub mod types;
use bank::{Name, balance::operations::Operation};

/// Внутрення обертка операции, сохраняющая дополнительно пользователя
/// (Мб костыль до того, как Name перерастет в User)
#[derive(Debug, PartialEq, Clone)]
pub struct OperationName(Operation, Name);
