use super::Storage;
use crate::{
    Name,
    balance::{Balance, BalanceManager, BalanceManagerError},
};
use std::collections::HashMap;

impl Storage {
    pub fn new() -> Self {
        Storage {
            accounts: HashMap::new(),
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

impl BalanceManager for Storage {
    fn deposit(&mut self, name: &Name, amount: i64) -> Result<(), BalanceManagerError> {
        if let Some(balance) = self.accounts.get_mut(name) {
            *balance += amount;
            Ok(())
        } else {
            Err(BalanceManagerError::UserNotFound(name.clone()))
        }
    }

    fn withdraw(&mut self, name: &Name, amount: i64) -> Result<(), BalanceManagerError> {
        if let Some(balance) = self.accounts.get_mut(name) {
            if balance.get_value() >= amount {
                *balance -= amount;
                Ok(())
            } else {
                Err(BalanceManagerError::NotEnoughMoney {
                    required: amount,
                    available: balance.get_value(),
                })
            }
        } else {
            Err(BalanceManagerError::UserNotFound(name.clone()))
        }
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

    #[test]
    fn test_remove_user() {
        let mut storage = Storage::new();
        storage.add_user("Bob".to_string());
        storage.deposit(&"Bob".to_string(), 100.into()).unwrap();

        let mut balance = Balance::new(0);
        balance += 100;

        assert_eq!(storage.remove_user(&"Bob".to_string()), Some(balance)); // удаляем и получаем баланс
        assert_eq!(storage.remove_user(&"Bob".to_string()), None); // второй раз — не найден
    }

    #[test]
    fn test_nonexistent_user() {
        let mut storage = Storage::new();

        // Депозит несуществующему пользователю
        assert!(storage.deposit(&"Dana".to_string(), 100.into()).is_err());

        // Снятие у несуществующего пользователя
        assert!(storage.withdraw(&"Dana".to_string(), 50.into()).is_err());

        // Баланс у несуществующего пользователя
        assert_eq!(storage.get_balance(&"Dana".to_string()), None);
    }
}
