use sqlx::PgPool;
use std::sync::Arc;

use crate::{
    data::state::{
        account::AccountStateRepo, transactions::TransactionStateRepo, user::UserStateRepo,
    },
    domain::{
        account::AccountRepository, transaction::TransactionRepository, user::UserRepository,
    },
    infrastructure::state::State,
};

pub mod sea;
pub mod state;

#[derive(Clone)]
pub enum Database {
    SEA(Arc<PgPool>),
    STATE(Arc<State>),
}

macro_rules! fn_get_repo {
    // ($name:ident, $train:ident, $repo_sea:ident, $repo_state:ident) => {
    //     pub fn $name(self: Arc<Self>) -> Box<dyn $train> {
    //         match self.as_ref() {
    //             Database::SEA(pool) => Box::new($repo_sea!("{}", pool.clone())),
    //             Database::STATE(state) => Box::new($repo_state(state.clone())),
    //         }
    //     }
    // };
    ($name:ident, $train:ident, todo, $repo_state:ident) => {
        pub fn $name(self: Arc<Self>) -> Box<dyn $train> {
            match self.as_ref() {
                Database::SEA(_) => todo!(),
                Database::STATE(state) => Box::new($repo_state(state.clone())),
            }
        }
    }; // ($name:ident, $train:ident, $repo_sea:ident, todo) => {
       //     pub fn $name(self: Arc<Self>) -> Box<dyn $train> {
       //         match self.as_ref() {
       //             Database::SEA(pool) => Box::new($repo_sea(pool.clone())),
       //             Database::STATE(state) => todo!(),
       //         }
       //     }
       // };
}

impl Database {
    fn_get_repo!(get_user_repo, UserRepository, todo, UserStateRepo);
    fn_get_repo!(get_account_repo, AccountRepository, todo, AccountStateRepo);
    fn_get_repo!(
        get_transaction_repo,
        TransactionRepository,
        todo,
        TransactionStateRepo
    );
}
