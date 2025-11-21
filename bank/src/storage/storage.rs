use super::Storage;
use crate::{Name, balance::Balance};
use std::collections::HashMap;

impl Storage {
    pub fn new() -> Self {
        Storage {
            accounts: HashMap::new(),
            __id_balance_gen: 1,
        }
    }
    pub fn add_user(&mut self, name: Name) -> Option<Balance> {
        if self.accounts.contains_key(&name) {
            None
        } else {
            self.accounts.insert(name, 0.into());
            Some(0.into())
        }
    }

    pub fn remove_user(&mut self, name: &Name) -> Option<Balance> {
        self.accounts.remove(name)
    }

    pub fn get_balance(&self, name: &Name) -> Option<&Balance> {
        self.accounts.get(name)
    }

    pub fn get_all(&self) -> Vec<(Name, &Balance)> {
        self.accounts.iter().map(|(n, b)| (n.clone(), b)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_user() {
        let mut storage = Storage::new();
        assert_eq!(storage.add_user("Alice".to_string()), Some(0.into())); // новый пользователь
        assert_eq!(storage.add_user("Alice".to_string()), None); // уже существует
    }
}
