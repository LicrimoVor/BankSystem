use crate::{
    domain::transaction::{Operation, Transaction},
    infrastructure::{error::ErrorApi, state::State},
};

impl Transaction {
    pub fn apply(self, state: &mut State) -> Result<(), ErrorApi> {
        Ok(())
    }
}
