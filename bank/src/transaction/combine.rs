use super::{Transaction, TxError};
use crate::Storage;
use std::ops::Add;

/// Комбинирование двух транзакций
/// ```ignore
/// let t1 = Deposit::new("a".into(), 10);
/// let t2 = Withdraw::new("a".into(), 5);
/// let t: TxCombinator<Deposit, Withdraw> = t1 + t2;
/// t.apply(&mut storage);
/// ```
#[derive(Debug, Clone)]
pub struct TxCombinator<T1: Transaction, T2: Transaction> {
    t1: T1,
    t2: T2,
}

impl<T1: Transaction, T2: Transaction> TxCombinator<T1, T2> {
    pub fn new(t1: T1, t2: T2) -> Self {
        Self { t1, t2 }
    }
}

impl<T1: Transaction, T2: Transaction> Transaction for TxCombinator<T1, T2> {
    fn apply(&self, storage: &mut Storage) -> Result<(), TxError> {
        self.t1.apply(storage)?;
        self.t2.apply(storage)?;
        Ok(())
    }
}

impl<T1: Transaction, T2: Transaction, Rhs: Transaction> Add<Rhs> for TxCombinator<T1, T2> {
    type Output = TxCombinator<TxCombinator<T1, T2>, Rhs>;

    fn add(self, rhs: Rhs) -> Self::Output {
        TxCombinator::new(self, rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Deposit, Withdraw};
    use super::*;
    use crate::Balance;
    use crate::storage::Storage;

    #[test]
    fn test_tx_combinator_invalid() {
        let mut storage = Storage::new();

        let t1 = Deposit::new("a".into(), 10);
        let t2 = Withdraw::new("a".into(), 5);
        let t: TxCombinator<Deposit, Withdraw> = t1 + t2;
        assert_eq!(t.apply(&mut storage), Err(TxError::InvalidAccount));
    }

    #[test]
    fn test_tx_combinator_valid() {
        let mut storage = Storage::new();
        storage.add_user('a'.into());

        let t1 = Deposit::new("a".into(), 10);
        let t2 = Withdraw::new("a".into(), 5);
        let t: TxCombinator<Deposit, Withdraw> = t1.clone() + t2.clone();
        assert_eq!(t.apply(&mut storage), Ok(()));
        assert_eq!(
            storage.get_balance(&"a".into()),
            Some(&Balance::new(5, vec![t1.into(), t2.into()]))
        );
    }

    #[test]
    fn test_tx_combinator_combination() {
        let t1 = Deposit::new("a".into(), 10);
        let t2 = Withdraw::new("a".into(), 5);
        let t: TxCombinator<Deposit, Withdraw> = t1 + t2;
        assert_eq!(t.apply(&mut Storage::new()), Err(TxError::InvalidAccount));
    }
}
