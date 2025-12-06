mod core;
pub mod files;
pub mod manager;
use crate::{Name, balance::Balance};
use std::collections::HashMap;

/// Структура хранилища
#[derive(Debug)]
pub struct Storage {
    /// Поле для хранения пользовательских балансов
    accounts: HashMap<Name, Balance>,

    /// поле для генерации уникальных id для баланса
    __id_balance_gen: u64,
}
