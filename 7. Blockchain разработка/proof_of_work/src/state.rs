use crate::block::Blockchain;
use crate::tx::Tx;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

// Простая имплеметация пула транзакций
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mempool {
    pub transactions: Vec<Tx>,
}

impl Mempool {
    pub fn new() -> Self {
        Mempool {
            transactions: vec![],
        }
    }

    pub fn push(&mut self, tx: Tx) {
        if tx.is_valid() {
            self.transactions.push(tx);
        }
    }

    pub fn get_txs(&self) -> Vec<Tx> {
        self.transactions.clone()
    }
}

pub struct AppState {
    pub blockchain: Mutex<Blockchain>,
    pub pool: Mutex<Mempool>,
}
