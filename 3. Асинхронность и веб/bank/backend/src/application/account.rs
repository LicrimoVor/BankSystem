use uuid::Uuid;

use crate::{
    data::{state::account::AccountStateRepo, Database},
    domain::{
        account::{Account, AccountRepository},
        user::User,
    },
    infrastructure::error::ErrorApi,
};

pub async fn create_account(
    db: Database,
    user: &User,
    amount: Option<f64>,
) -> Result<Account, ErrorApi> {
    let mut repo = match db {
        Database::STATE(state) => AccountStateRepo(state),
        Database::SEA(_) => todo!(),
    };

    repo.create(user, amount).await
}

pub async fn get_account_by_id(db: Database, id: Uuid) -> Option<Account> {
    let repo = match db {
        Database::STATE(state) => AccountStateRepo(state),
        Database::SEA(_) => todo!(),
    };

    repo.get_by_id(id).await
}

pub async fn delete_account(db: Database, user: &User, account_id: Uuid) -> Result<(), ErrorApi> {
    let mut repo = match db {
        Database::STATE(state) => AccountStateRepo(state),
        Database::SEA(_) => todo!(),
    };

    let Some(account) = repo.get_by_id(account_id).await else {
        return Err(ErrorApi::NotFound("account".to_string()));
    };

    if account.user_id() != user.id() {
        return Err(ErrorApi::Forbidden);
    }

    repo.delete(&account).await
}
