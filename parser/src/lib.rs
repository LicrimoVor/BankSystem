#![deny(unreachable_pub)]
pub mod errors;
pub mod from;
pub mod to;
pub(crate) mod types;
use bank::{Name, balance::operations::Operation};
use clap::ValueEnum;

/// ## Тип файла
///
/// ### Возможные значения
/// - [FileType::CSV] - Csv формат файла
/// - [FileType::TXT] - Txt формат файла
/// - [FileType::BIN] - Bin формат файла
#[derive(Debug, PartialEq, Clone, ValueEnum)]
pub enum FileType {
    /// Csv формат файла
    CSV,
    /// Txt формат файла
    TXT,
    /// Bin формат файла
    BIN,
}

/// Внутрення обертка операции, сохраняющая дополнительно пользователя
/// (Мб костыль до того, как Name перерастет в User)
#[derive(Debug, PartialEq, Clone)]
pub struct OperationName(Operation, Name);
