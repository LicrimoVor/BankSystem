use super::{Balance, BalanceOp};
use crate::Storage;

/// Аналитика баланса
pub struct Analitic;

impl Analitic {
    /// Наиболее активный пользователь
    pub fn find_most_active(storage: &Storage) -> Option<(String, &Balance)> {
        let accounts = storage.get_all();
        if accounts.is_empty() {
            return None;
        }
        let result = accounts
            .into_iter()
            .map(|(n, b)| {
                let a: i64 = b
                    .get_history()
                    .iter()
                    .map(|op| match op {
                        BalanceOp::Deposit(v) => *v,
                        BalanceOp::Withdraw(v) => *v,
                        _ => 0,
                    })
                    .sum();
                (a, (n, b))
            })
            .max_by(|a, b| a.0.cmp(&b.0))
            .unwrap()
            .1
            .clone();

        Some(result)
    }

    /// Наиболее богатый
    pub fn find_most_rich(storage: &Storage) -> Option<(String, &Balance)> {
        let accounts = storage.get_all();
        if accounts.is_empty() {
            return None;
        }
        let result = accounts
            .into_iter()
            .max_by(|a, b| a.1.get_value().cmp(&b.1.get_value()))
            .unwrap();
        Some(result)
    }
}

#[cfg(test)]
mod test {
    use super::super::manager::BalanceManager;
    use super::*;

    fn get_storage() -> Storage {
        let mut storage = Storage::new();
        storage.add_user("a".into());
        storage.add_user("b".into());

        let _ = storage.deposit(&"a".into(), 15);
        let _ = storage.deposit(&"b".into(), 20);
        let _ = storage.withdraw(&"a".into(), 15);
        storage
    }

    #[test]
    fn test_find_most_active_none() {
        let storage = super::Storage::new();
        assert_eq!(Analitic::find_most_active(&storage), None);
    }

    #[test]
    fn test_find_most_active_some() {
        let storage = get_storage();
        let balance = storage.get_balance(&"a".into()).unwrap();
        assert_eq!(
            Analitic::find_most_active(&storage),
            Some(("a".into(), balance))
        );
    }

    #[test]
    fn test_find_most_rich_none() {
        let storage = super::Storage::new();
        assert_eq!(Analitic::find_most_rich(&storage), None);
    }

    #[test]
    fn test_find_most_rich_some() {
        let storage = get_storage();
        let balance = storage.get_balance(&"b".into()).unwrap();
        assert_eq!(
            Analitic::find_most_rich(&storage),
            Some(("b".into(), balance))
        );
    }
}
