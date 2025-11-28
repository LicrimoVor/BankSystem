/// Ошибки операции
pub enum OperationError {
    /// Недостаточно средств
    NotEnoughMoney {
        required: u64,
        available: BalanceSize,
    },

    /// Неверная операция
    InvalidOperation(String),

    /// Ошибка парсинга
    ParseError(String),

    /// Перевышен лимит
    OverLimitInt64,
}
