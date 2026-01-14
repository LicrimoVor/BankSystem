use crate::balance::operations::OperationError;

/// # Ошибки
/// - [TxError::InsufficientFunds] - Недостаточно средств
/// - [TxError::InvalidAccount] - Не найден счет
/// - [TxError::ManageError] - Ошибка работы с балансом
#[derive(Debug, PartialEq)]
pub enum TxError {
    InsufficientFunds,
    InvalidAccount,
    OperationError(OperationError),
}
