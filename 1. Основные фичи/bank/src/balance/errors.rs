use super::operations::OperationError;

/// Ошибки работы с балансом
#[derive(Debug)]
pub enum BalanceError {
    InvalidParseOperation(OperationError),
    InvalidParseBalance(String),
}
