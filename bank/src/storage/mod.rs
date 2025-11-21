pub mod files;
pub mod manager;
pub mod storage;
use crate::{Name, balance::Balance};
use std::collections::HashMap;

/// Структура хранилища
#[derive(Debug)]
pub struct Storage {
    accounts: HashMap<Name, Balance>,

    /// поле для генерации уникальных id для баланса
    __id_balance_gen: u64,
}
