use super::Storage;
use crate::{
    Name,
    balance::{BalanceManager, BalanceManagerError, BalanceOp},
};

impl BalanceManager for Storage {
    fn deposit(&mut self, name: &Name, amount: i64) -> Result<(), BalanceManagerError> {
        if let Some(balance) = self.accounts.get_mut(name) {
            let deposit = BalanceOp::Deposit(amount);
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

    fn transfer(&mut self, from: &Name, to: &Name, amount: i64) -> Result<(), BalanceManagerError> {
        if let [Some(from_balance), Some(to_balance)] = self.accounts.get_disjoint_mut([from, to]) {
            if from_balance.get_value() >= amount {
                *from_balance -= amount;
                *to_balance += amount;
                Ok(())
            } else {
                Err(BalanceManagerError::NotEnoughMoney {
                    required: amount,
                    available: from_balance.get_value(),
                })
            }
        } else {
            if self.accounts.contains_key(from) {
                Err(BalanceManagerError::UserNotFound(to.clone()))
            } else {
                Err(BalanceManagerError::UserNotFound(from.clone()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::balance::Balance;

    #[test]
    fn test_remove_user() {
        let mut storage = Storage::new();
        storage.add_user("Bob".to_string());
        storage.deposit(&"Bob".to_string(), 100.into()).unwrap();

        let mut balance = Balance::default();
        balance += 100;

        assert_eq!(storage.remove_user(&"Bob".to_string()), Some(balance)); // удаляем и получаем баланс
        assert_eq!(storage.remove_user(&"Bob".to_string()), None); // второй раз — не найден
    }

    #[test]
    fn test_nonexistent_user() {
        let mut storage = Storage::new();

        assert!(storage.deposit(&"Dana".to_string(), 100.into()).is_err());
        assert!(storage.withdraw(&"Dana".to_string(), 50.into()).is_err());
        assert_eq!(storage.get_balance(&"Dana".to_string()), None);
    }
}
