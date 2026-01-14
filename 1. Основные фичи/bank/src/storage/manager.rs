use super::Storage;
use crate::{
    Name,
    balance::{
        manager::{BalanceManager, BalanceManagerError},
        operations::{Operation, OperationAmount},
    },
};

impl BalanceManager for Storage {
    fn deposit(&mut self, name: &Name, amount: OperationAmount) -> Result<(), BalanceManagerError> {
        let id = self._get_id_balance();
        let Some(balance) = self.accounts.get_mut(name) else {
            Err(BalanceManagerError::UserNotFound(name.clone()))?
        };

        Operation::deposit(id, amount)
            .apply(balance)
            .map_err(BalanceManagerError::OperationError)?;

        Ok(())
    }

    fn withdraw(
        &mut self,
        name: &Name,
        amount: OperationAmount,
    ) -> Result<(), BalanceManagerError> {
        let id = self._get_id_balance();
        let Some(balance) = self.accounts.get_mut(name) else {
            Err(BalanceManagerError::UserNotFound(name.clone()))?
        };

        Operation::withdraw(id, amount)
            .apply(balance)
            .map_err(BalanceManagerError::OperationError)?;

        Ok(())
    }

    fn transfer(
        &mut self,
        from: &Name,
        to: &Name,
        amount: OperationAmount,
    ) -> Result<(), BalanceManagerError> {
        let id = self._get_id_balance();
        if let [Some(balance_from), Some(balance_to)] = self.accounts.get_disjoint_mut([from, to]) {
            let operation_from = Operation::transfer(id, to.clone(), amount, false);
            let operation_to = Operation::transfer(id, from.clone(), amount, true);
            operation_from
                .apply(balance_from)
                .map_err(BalanceManagerError::OperationError)?;
            operation_to
                .apply(balance_to)
                .map_err(BalanceManagerError::OperationError)?;

            Ok(())
        } else if self.accounts.contains_key(from) {
            Err(BalanceManagerError::UserNotFound(to.clone()))
        } else {
            Err(BalanceManagerError::UserNotFound(from.clone()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_user() {
        let mut storage = Storage::new();
        storage.add_user("Bob".to_string());
        storage.deposit(&"Bob".to_string(), 100).unwrap();

        let res = storage.remove_user(&"Bob".to_string());
        assert!(res.is_some()); // удаляем и получаем баланс
        let balance = res.unwrap();
        assert_eq!(balance.get_value(), 100);

        let res = storage.remove_user(&"Bob".to_string());
        assert!(res.is_none()); // второй раз — не найден
    }

    #[test]
    fn test_nonexistent_user() {
        let mut storage = Storage::new();

        assert!(storage.deposit(&"Dana".to_string(), 100).is_err());
        assert!(storage.withdraw(&"Dana".to_string(), 50).is_err());
        assert_eq!(storage.get_balance(&"Dana".to_string()), None);
    }
}
