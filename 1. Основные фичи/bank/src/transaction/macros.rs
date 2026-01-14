#[macro_export]
macro_rules! tx_chain {
    ( $first:expr $(, $rest:expr )* $(,)? ) => {{
        let tx = $first;
        $(
            let tx = $crate::transaction::TxCombinator::new(tx, $rest);
        )*
        tx
    }};
}

#[macro_export]
macro_rules! impl_add {
    ( $( ($lhs:ty, $rhs:ty) ),* ) => {
        $(
            impl std::ops::Add<$rhs> for $lhs {
                type Output = $crate::transaction::TxCombinator<$lhs, $rhs>;

                fn add(self, rhs: $rhs) -> Self::Output {
                    $crate::transaction::TxCombinator::new(self, rhs)
                }
            }
        )*
    };
}

/// Макрос для автоматического написания оператора `+` для транзакций
#[macro_export(local_inner_macros)]
macro_rules! impl_add_trait {
    ( $( $lhs:ty ),* ) => {
        $(
            impl<Rhs: $crate::transaction::Transaction> std::ops::Add<Rhs> for $lhs {
                type Output = $crate::transaction::TxCombinator<$lhs, Rhs>;

                fn add(self, rhs: Rhs) -> Self::Output {
                    $crate::transaction::TxCombinator::new(self, rhs)
                }
            }
        )*
    };
}
