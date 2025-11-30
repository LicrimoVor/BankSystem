pub mod errors;
pub mod from;
pub mod to;
use bank::{Name, balance::operations::Operation};

/// Тип файла
#[derive(Debug, PartialEq, Clone)]
pub enum FileType {
    CSV,
    TXT,
    BIN,
}

/// Внутрення обертка операции, сохраняющая дополнительно пользователя
/// (Мб костыль до того, как Name перерастет в User)
#[derive(Debug, PartialEq, Clone)]
pub struct OperationName(Operation, Name);
