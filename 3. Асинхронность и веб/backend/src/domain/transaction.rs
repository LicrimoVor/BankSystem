use uuid::Uuid;

pub enum Operation {
    DEPOSIT,
    WITHDRAWAL,
    TRANSFER,
}

pub struct Transaction {
    operation: Operation,
    amount: f64,
    from: Option<Uuid>,
    to: Option<Uuid>,
}

impl Transaction {
    pub fn deposit(amount: f64, to: Uuid) -> Transaction {
        Transaction {
            operation: Operation::DEPOSIT,
            amount,
            from: None,
            to: Some(to),
        }
    }

    pub fn withdrawal(amount: f64, from: Uuid) -> Transaction {
        Transaction {
            operation: Operation::WITHDRAWAL,
            amount,
            from: Some(from),
            to: None,
        }
    }

    pub fn transfer(amount: f64, from: Uuid, to: Uuid) -> Transaction {
        Transaction {
            operation: Operation::TRANSFER,
            amount,
            from: Some(from),
            to: Some(to),
        }
    }
}
