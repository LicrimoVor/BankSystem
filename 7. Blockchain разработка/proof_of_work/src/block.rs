use crate::libs::leading_zeros;
use crate::tx::Tx;
use anyhow::Result;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use serde::{Deserialize, Serialize};
use thiserror::Error;

// Мы инкапсулируем реализацию конкретного значения хэша,
// чтобы иметь возможность легко изменить его в будущем
pub(crate) type BlockHash = [u8; 32];
// Урвень сложности блокчейн системы
pub(crate) const DIFFICULTY: u32 = 2;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Block {
    pub index: u64,                // позицию в цепи
    pub nonce: u64,                // число, найденное майнерами для соблюдения сложности
    pub previous_block: BlockHash, // хэш предыдущего блока (нет у генезис-блока)
    pub hash: BlockHash,           // хэш текущего блока
    pub txs: Vec<Tx>,              // список транзакций с отправителем, получателем и суммой
}

impl Block {
    // При создании нового блока. Хеш-значение рассчитывается автоматически.
    pub fn new(index: u64, nonce: u64, previous_block: BlockHash, txs: Vec<Tx>) -> Self {
        let mut block = Block {
            index,
            nonce,
            previous_block,
            hash: BlockHash::default(),
            txs,
        };
        block.hash = block.calculate_hash();

        block
    }

    // Рассчитывается хеш-значение блока
    pub fn calculate_hash(&self) -> BlockHash {
        let mut hashable_data = self.clone();
        hashable_data.hash = BlockHash::default();
        let serialized = serde_json::to_string(&hashable_data).unwrap();

        let mut block_hash: BlockHash = <[u8; 32]>::default();
        let mut hasher = Sha256::new();

        hasher.input_str(&serialized);
        hasher.result(&mut block_hash);

        block_hash
    }
}

// Типы ошибок, возвращаемые при попытке добавить блоки с недопустимыми полями
#[derive(Error, PartialEq, Debug)]
pub(crate) enum BlockchainError {
    #[error("Invalid previous_hash")]
    PreviousHashMismatch,

    #[error("Invalid hash")]
    InvalidHash,

    #[error("Invalid difficulty")]
    IncorrectDifficulty,
}

// Структура блокчейн содержит все существующие блоки
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        // При первом создании блокчейн, добавляется блок генезиса
        let genesis_block = Blockchain::create_genesis_block();
        let blocks = vec![genesis_block];

        Blockchain { blocks }
    }

    pub fn create_genesis_block() -> Block {
        let index = 0;
        let nonce = 0;
        let previous_hash = BlockHash::default();
        let transactions = Vec::new();

        Block::new(index, nonce, previous_hash, transactions)
    }

    // Пытается добавить новый блок в блокчейн
    // Проверяется соответствие значений нового блока состоянию блокчейна
    pub fn add_block(&mut self, block: Block) -> Result<()> {
        let blocks = &self.blocks;
        let last = &self.blocks[blocks.len() - 1];
        // Проверка корректности указателя на предидущий блок
        if block.previous_block != last.hash {
            return Err(BlockchainError::PreviousHashMismatch.into());
        }

        // Проверка соответствия хеша транзакций
        if block.hash != block.calculate_hash() {
            return Err(BlockchainError::InvalidHash.into());
        }

        // Проверка уровня сложности
        if leading_zeros(&block.hash) < DIFFICULTY {
            return Err(BlockchainError::IncorrectDifficulty.into());
        }

        self.blocks.push(block);

        Ok(())
    }

    pub fn get_blocks(&self) -> Vec<Block> {
        self.blocks.clone()
    }
}
