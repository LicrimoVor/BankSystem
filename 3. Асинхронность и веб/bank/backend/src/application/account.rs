use std::sync::Arc;

use uuid::Uuid;

use crate::{
    data::Database,
    domain::{account::Account, user::User},
    infrastructure::error::ErrorApi,
};

pub async fn create_account(
    db: Arc<Database>,
    user: &User,
    amount: Option<f64>,
) -> Result<Account, ErrorApi> {
    let mut repo = db.get_account_repo();
    repo.create(user, amount).await
}

pub async fn get_account_by_id(db: Arc<Database>, id: Uuid) -> Option<Account> {
    let repo = db.get_account_repo();
    repo.get_by_id(id).await
}

pub async fn delete_account(
    db: Arc<Database>,
    user: &User,
    account_id: Uuid,
) -> Result<(), ErrorApi> {
    let mut repo = db.get_account_repo();

    let Some(account) = repo.get_by_id(account_id).await else {
        return Err(ErrorApi::NotFound("account".to_string()));
    };

    if account.user_id() != user.id() {
        return Err(ErrorApi::Forbidden);
    }

    repo.delete(&account).await
}
