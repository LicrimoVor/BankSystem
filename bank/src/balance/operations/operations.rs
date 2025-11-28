use super::{Status, balance::BalanceOp};
use std::time::{SystemTime, UNIX_EPOCH};

/// Операция баланса
#[derive(Debug, Clone)]
pub struct Operation {
    id: u64,
    timestamp: u64,

    pub tx_type: BalanceOp,
    pub status: Status,
    pub description: String,
}

impl Operation {
    pub fn new(id: u64, tx_type: BalanceOp, description: Option<String>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Установите актуальное время")
            .as_secs();

        Self {
            id,
            tx_type,
            timestamp,
            status: Status::PENDING,
            description: description.unwrap_or(format!("Record number #{}", id)),
        }
    }

    /// Создает операцию депозита
    pub fn deposit(id: u64, amount: u64) -> Self {
        Self::new(id, BalanceOp::Deposit(amount), None)
    }

    /// Создает операцию снятия
    pub fn withdraw(id: u64, amount: u64) -> Self {
        Self::new(id, BalanceOp::Withdraw(amount), None)
    }

    /// Создает операцию перевода
    pub fn transfer(id: u64, name: String, amount: u64, is_to: bool) -> Self {
        Self::new(id, BalanceOp::Transfer(name, amount, is_to), None)
    }

    /// Создает операцию закрытия
    pub fn close(id: u64) -> Self {
        Self::new(id, BalanceOp::Close, None)
    }

    /// Устанавливает статус операции
    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }
}
