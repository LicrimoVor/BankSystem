use tightness::{bound, ConstructionError};

bound!(pub TradingPair: String where |s| {
    let parts: Vec<&str> = s.split('/').collect();
    parts.len() == 2 &&
    !parts[0].is_empty() &&
    !parts[1].is_empty() &&
    parts[0].chars().all(|c| c.is_alphanumeric()) &&
    parts[1].chars().all(|c| c.is_alphanumeric())
});

#[derive(Debug, Clone)]
pub struct TradingAccount {
    balance: i64,
    orders: Vec<Order>,
}

#[derive(Debug, Clone)]
pub struct Order {
    amount: i64,
    order_type: OrderType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OrderType {
    Buy,
    Sell,
}

impl TradingAccount {
    pub fn new(initial_balance: i64) -> Self {
        Self {
            balance: initial_balance,
            orders: Vec::new(),
        }
    }

    pub fn balance(&self) -> i64 {
        self.balance
    }

    pub fn place_order(&mut self, order: Order) -> Result<(), String> {
        match order.order_type {
            OrderType::Buy => {
                if self.balance >= order.amount {
                    self.balance -= order.amount;
                    self.orders.push(order);
                    Ok(())
                } else {
                    Err("Insufficient balance".to_string())
                }
            }
            OrderType::Sell => {
                if let Some(new_balance) = self.balance.checked_add(order.amount) {
                    self.balance = new_balance;
                    self.orders.push(order);
                    Ok(())
                } else {
                    Err("Sell order would cause overflow".to_string())
                }
            }
        }
    }
}

fn main() -> Result<(), ConstructionError<String>> {
    let pair = TradingPair::new("BTC/USD".to_string())?;
    // Получаем базу и котировку напрямую из строки.
    let parts: Vec<&str> = pair.split('/').collect();
    let base = parts[0];
    let quote = parts[1];
    println!("Base: {}, Quote: {}", base, quote);

    let invalid = TradingPair::new("INVALID".to_string());
    assert!(invalid.is_err());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn order_strategy() -> impl Strategy<Value = Order> {
        (
            any::<i64>(),
            prop_oneof![Just(OrderType::Buy), Just(OrderType::Sell),],
        )
            .prop_map(|(amount, order_type)| Order {
                amount: amount.abs(),
                order_type,
            })
    }

    proptest! {
        #[test]
        fn balance_never_negative(
            initial_balance in 0i64..1_000_000,
            operations in prop::collection::vec(order_strategy(), 0..100)
        ) {
            let mut account = TradingAccount::new(initial_balance);

            for order in operations {
                let _ = account.place_order(order);
                prop_assert!(
                    account.balance() >= 0,
                    "Balance should never be negative, but got {}",
                    account.balance()
                );
            }
        }
    }

    proptest! {
        #[test]
        fn total_operations_match_balance(
            initial_balance in 0i64..1_000_000,
            operations in prop::collection::vec(order_strategy(), 0..100)
        ) {
            let mut account = TradingAccount::new(initial_balance);
            let mut expected_balance = initial_balance;

            for order in operations {
                match order.order_type {
                    OrderType::Buy => {
                        if expected_balance >= order.amount {
                            expected_balance -= order.amount;
                            let _ = account.place_order(order);
                        }
                    }
                    OrderType::Sell => {
                        expected_balance += order.amount;
                        let _ = account.place_order(order);
                    }
                }

                prop_assert_eq!(
                    account.balance(),
                    expected_balance,
                    "Balance should match expected value"
                );
            }
        }
    }
}
