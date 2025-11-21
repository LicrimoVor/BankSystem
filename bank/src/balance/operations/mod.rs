use std::time::{SystemTime, UNIX_EPOCH};
mod close;
mod deposit;
mod operations;
mod transfer;
mod withdraw;

pub use close::Close;
pub use deposit::Deposit;
use operations::BalanceOp;
pub use transfer::Transfer;
pub use withdraw::Withdraw;

pub enum Status {
    FAILURE,
    PENDING,
    SUCCESS,
}

struct Operation {
    id: u64,
    tx_type: BalanceOp,
    timestamp: u64,
    status: Status,
    description: String,
}

impl Operation {
    fn new(id: u64, tx_type: BalanceOp, description: Option<String>) -> Self {
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

    fn deposit(id: u64, amount: u64) -> Self {
        Self::new(0, BalanceOp::Deposit(amount), None)
    }

    fn withdraw(id: u64, amount: u64) -> Self {
        Self::new(0, BalanceOp::Withdraw(amount), None)
    }

    fn transfer(id: u64, amount: u64) -> Self {
        Self::new(0, BalanceOp::Transfer(0, amount), None)
    }
}
