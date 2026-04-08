use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum LeverError {
    #[error("Invalid instruction data")]
    InvalidInstruction = 0,

    #[error("Missing signature")]
    MissingSignature = 1,

    #[error("Invalid lever authority PDA")]
    InvalidAuthorityPda = 2,

    #[error("Account must be writable")]
    NotWritable = 3,

    #[error("Invalid lever program account")]
    InvalidLeverProgramAccount = 4,
}

impl From<LeverError> for ProgramError {
    fn from(e: LeverError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
