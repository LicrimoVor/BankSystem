pub mod sql;
pub mod state;
pub mod transaction;

use crate::{
    domain::{
        account::AccountRepository, token::RefreshTokenRepository,
        transaction::TransactionRepository, user::UserRepository,
    },
    infrastructure::{error::ErrorApi, state::State},
};
use sql::{
    account::AccountSQLRepo, token::RefreshTokenSQLRepo, transactions::TransactionSQLRepo,
    user::UserSQLRepo, DBTransactionSQL,
};
use sqlx::PgPool;
use state::{
    account::AccountStateRepo, token::RefreshTokenStateRepo, transactions::TransactionStateRepo,
    user::UserStateRepo, DBTransactionState,
};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum Database {
    PgSQL(Arc<PgPool>),
    STATE(Arc<State>),
}

macro_rules! fn_get_repo {
    ($name:ident, $train:ident, $repo_sql:ident, $repo_state:ident) => {
        pub fn $name(self: Arc<Self>) -> Box<dyn $train> {
            match self.as_ref() {
                Database::PgSQL(pool) => Box::new($repo_sql(pool.clone())),
                Database::STATE(state) => Box::new($repo_state(state.clone())),
            }
        }
    };
}

impl Database {
    pub async fn transaction(
        self: Arc<Self>,
    ) -> Result<Box<dyn transaction::DBTransaction>, ErrorApi> {
        match self.as_ref() {
            Database::PgSQL(pool) => match DBTransactionSQL::new(pool.clone()).await {
                Ok(tx) => Ok(Box::new(tx) as Box<dyn transaction::DBTransaction>),
                Err(err) => Err(err),
            },
            Database::STATE(_) => {
                Ok(Box::new(DBTransactionState::new()) as Box<dyn transaction::DBTransaction>)
            }
        }
    }
}

impl Database {
    fn_get_repo!(get_user_repo, UserRepository, UserSQLRepo, UserStateRepo);
    fn_get_repo!(
        get_account_repo,
        AccountRepository,
        AccountSQLRepo,
        AccountStateRepo
    );
    fn_get_repo!(
        get_transaction_repo,
        TransactionRepository,
        TransactionSQLRepo,
        TransactionStateRepo
    );
    fn_get_repo!(
        get_refresh_token_repo,
        RefreshTokenRepository,
        RefreshTokenSQLRepo,
        RefreshTokenStateRepo
    );
}
