use anchor_lang::prelude::*;

#[error_code]
pub enum OracleError {
    #[msg("Only oracle admin may call this instruction")]
    Unauthorized,
    #[msg("Price must be greater than zero")]
    InvalidPrice,
    #[msg("Slot must be greater than last updated slot")]
    InvalidSlot,
}
