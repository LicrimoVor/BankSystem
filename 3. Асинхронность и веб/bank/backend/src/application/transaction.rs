use std::sync::Arc;
use uuid::Uuid;

use crate::{
    data::Database,
    domain::{transaction::Transaction, user::User},
    infrastructure::error::ErrorApi,
};

pub async fn deposit(
    db: Arc<Database>,
    user: &User,
    account_id: Uuid,
    amount: f64,
) -> Result<Transaction, ErrorApi> {
    let mut repo_acc = db.clone().get_account_repo();
    let mut repo_tran = db.get_transaction_repo();
    let Some(mut account) = repo_acc.get_by_id(account_id).await else {
        return Err(ErrorApi::Validation("Account not found".to_string()));
    };
    if account.user_id() != user.id() {
        return Err(ErrorApi::Validation(
            "Account does not belong to user".to_string(),
        ));
    }
    account.set_balance(*account.balance() + amount);

    // ТРАНЗАКЦИЯ!
    repo_acc.update(&account).await?;
    repo_tran.create_deposit(amount, &account).await
    // ТРАНЗАКЦИЯ!
}

pub async fn withdraw(
    db: Arc<Database>,
    user: &User,
    account_id: Uuid,
    amount: f64,
) -> Result<Transaction, ErrorApi> {
    let mut repo_acc = db.clone().get_account_repo();
    let mut repo_tran = db.get_transaction_repo();
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
    repo_acc.update(&account).await?;
    repo_tran.create_withdrawal(amount, &account).await
    // ТРАНЗАКЦИЯ!
}

pub async fn transfer(
    db: Arc<Database>,
    user: &User,
    from_account_id: Uuid,
    to_account_id: Uuid,
    amount: f64,
) -> Result<Transaction, ErrorApi> {
    let mut repo_acc = db.clone().get_account_repo();
    let mut repo_tran = db.get_transaction_repo();
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
    repo_acc.update(&from_account).await?;
    repo_acc.update(&to_account).await?;
    repo_tran
        .create_transfer(amount, &from_account, &to_account)
        .await
    // ТРАНЗАКЦИЯ!
}
