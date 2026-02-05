pub mod sql;
pub mod state;
pub mod transaction;

use crate::{
    data::{sql::course::CourseSQLRepo, state::course::CourseStateRepo},
    domain::{
        account::AccountRepository, course::CourseRepository, token::RefreshTokenRepository,
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
use transaction::DBTransaction;

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
    pub async fn transaction(self: Arc<Self>) -> Result<DBTransaction, ErrorApi> {
        match self.as_ref() {
            Database::PgSQL(pool) => match DBTransactionSQL::new(pool.clone()).await {
                Ok(tx) => Ok(DBTransaction::SQL(tx)),
                Err(err) => Err(err),
            },
            Database::STATE(_) => Ok(DBTransaction::STATE(DBTransactionState::new())),
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
    fn_get_repo!(
        get_course_repo,
        CourseRepository,
        CourseSQLRepo,
        CourseStateRepo
    );
}
