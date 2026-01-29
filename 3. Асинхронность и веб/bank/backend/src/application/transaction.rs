use uuid::Uuid;

use crate::{
    data::{
        state::{account::AccountStateRepo, transactions::TransactionStateRepo},
        Database,
    },
    domain::{
        account::AccountRepository,
        transaction::{Transaction, TransactionRepository},
        user::User,
    },
    infrastructure::error::ErrorApi,
};

const MIN_AMOUNT: f64 = 0.01;

pub async fn deposit(
    db: Database,
    user: &User,
    account_id: Uuid,
    amount: f64,
) -> Result<Transaction, ErrorApi> {
    let (mut repo_acc, mut repo_tran) = match db {
        Database::STATE(state) => (AccountStateRepo(state.clone()), TransactionStateRepo(state)),
        Database::SEA(_) => todo!(),
    };
    let Some(mut account) = repo_acc.get_by_id(account_id).await else {
        return Err(ErrorApi::Validation("Account not found".to_string()));
    };
    if account.user_id() != user.id() {
        return Err(ErrorApi::Validation(
            "Account does not belong to user".to_string(),
        ));
    }
    if *account.balance() < amount {
        return Err(ErrorApi::Validation(
            "Account balance is not enough".to_string(),
        ));
    }
    account.set_balance(*account.balance() + amount);

    // ТРАНЗАКЦИЯ!
    repo_acc.update(&account).await;
    repo_tran.create_deposit(amount, &account).await
    // ТРАНЗАКЦИЯ!
}

pub async fn withdraw(
    db: Database,
    user: &User,
    account_id: Uuid,
    amount: f64,
) -> Result<Transaction, ErrorApi> {
    if amount < MIN_AMOUNT {
        return Err(ErrorApi::Validation(
            "Transaction amount must be at least 0.01".to_string(),
        ));
    }

    let (mut repo_acc, mut repo_tran) = match db {
        Database::STATE(state) => (AccountStateRepo(state.clone()), TransactionStateRepo(state)),
        Database::SEA(_) => todo!(),
    };
    let Some(mut account) = repo_acc.get_by_id(account_id).await else {
        return Err(ErrorApi::Validation("Account not found".to_string()));
    };
    if account.user_id() != user.id() {
        return Err(ErrorApi::Validation(
            "Account does not belong to user".to_string(),
        ));
    }
    if *account.balance() < amount {
        return Err(ErrorApi::Validation(
            "Account balance is not enough".to_string(),
        ));
    }
    account.set_balance(*account.balance() - amount);

    // ТРАНЗАКЦИЯ!
    repo_acc.update(&account).await;
    repo_tran.create_withdrawal(amount, &account).await
    // ТРАНЗАКЦИЯ!
}

pub async fn transfer(
    db: Database,
    user: &User,
    from_account_id: Uuid,
    to_account_id: Uuid,
    amount: f64,
) -> Result<Transaction, ErrorApi> {
    if amount < MIN_AMOUNT {
        return Err(ErrorApi::Validation(
            "Transaction amount must be at least 0.01".to_string(),
        ));
    }

    let (mut repo_acc, mut repo_tran) = match db {
        Database::STATE(state) => (AccountStateRepo(state.clone()), TransactionStateRepo(state)),
        Database::SEA(_) => todo!(),
    };
    let Some(mut from_account) = repo_acc.get_by_id(from_account_id).await else {
        return Err(ErrorApi::Validation("Account not found".to_string()));
    };
    if from_account.user_id() != user.id() {
        return Err(ErrorApi::Validation(
            "Account does not belong to user".to_string(),
        ));
    }
    if *from_account.balance() < amount {
        return Err(ErrorApi::Validation(
            "Account balance is not enough".to_string(),
        ));
    }

    let Some(mut to_account) = repo_acc.get_by_id(to_account_id).await else {
        return Err(ErrorApi::Validation("Account not found".to_string()));
    };

    from_account.set_balance(*from_account.balance() - amount);
    to_account.set_balance(*to_account.balance() + amount);

    // ТРАНЗАКЦИЯ!
    repo_acc.update(&from_account).await;
    repo_acc.update(&to_account).await;
    repo_tran
        .create_transfer(amount, &from_account, &to_account)
        .await
    // ТРАНЗАКЦИЯ!
}
