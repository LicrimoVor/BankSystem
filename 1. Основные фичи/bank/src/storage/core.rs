use super::Storage;
use crate::{Name, balance::Balance};
use std::collections::HashMap;

impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}

impl Storage {
    /// Создание нового хранилища
    pub fn new() -> Self {
        Storage {
            accounts: HashMap::new(),
            __id_balance_gen: 1,
        }
    }

    /// Добавить пользователя
    pub fn add_user(&mut self, name: Name) -> Option<&Balance> {
        if self.accounts.contains_key(&name) {
            None
        } else {
            self.accounts.insert(name.clone(), 0.into());

            Some(self.accounts.get(&name).unwrap())
        }
    }

    /// Удалить пользователя
    pub fn remove_user(&mut self, name: &Name) -> Option<Balance> {
        self.accounts.remove(name)
    }

    /// Получить баланс
    pub fn get_balance(&self, name: &Name) -> Option<&Balance> {
        self.accounts.get(name)
    }

    /// Получить всех пользователей и их балансы
    pub fn get_all(&self) -> Vec<(Name, &Balance)> {
        self.accounts.iter().map(|(n, b)| (n.clone(), b)).collect()
    }

    pub(crate) fn _get_id_balance(&mut self) -> u64 {
        let id = self.__id_balance_gen;
        self.__id_balance_gen += 1;
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_user() {
        let mut storage = Storage::new();
        assert_eq!(storage.add_user("Alice".to_string()), Some(&0.into())); // новый пользователь
        assert_eq!(storage.add_user("Alice".to_string()), None); // уже существует
    }
}
