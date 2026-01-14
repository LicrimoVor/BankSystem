use super::super::BalanceSize;

/// Ошибки операции
#[derive(Debug, Clone, PartialEq)]
pub enum OperationError {
    /// Недостаточно средств
    NotEnoughMoney {
        required: u64,
        available: BalanceSize,
    },

    /// Неверная операция
    InvalidOperation(String),

    /// Неверный статус
    InvalidStatus,

    /// Ошибка парсинга
    ParseError(String),

    /// Перевышен лимит
    OverLimitSize,
}
