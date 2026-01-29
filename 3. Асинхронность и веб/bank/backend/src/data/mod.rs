use sqlx::PgPool;
use std::sync::Arc;

use crate::infrastructure::state::State;

pub mod sea;
pub mod state;

#[derive(Clone)]
pub enum Database {
    SEA(PgPool),
    STATE(Arc<State>),
}
