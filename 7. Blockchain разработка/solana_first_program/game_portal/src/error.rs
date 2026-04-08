use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum PortalError {
    #[error("Invalid instruction data")]
    InvalidInstruction = 0,

    #[error("Missing required signature")]
    MissingSignature = 1,

    #[error("Cheating is not allowed: Open/Close only via Lever (CPI)")]
    CheatingNotAllowed = 2,

    #[error("Invalid portal PDA")]
    InvalidPortalPda = 3,

    #[error("Invalid lever program account")]
    InvalidLeverProgram = 4,

    #[error("Invalid lever authority PDA or signature")]
    InvalidLeverAuthority = 5,

    #[error("Already initialized")]
    AlreadyInitialized = 6,

    #[error("Not initialized")]
    NotInitialized = 7,

    #[error("Not the owner")]
    NotOwner = 8,

    #[error("Account must be writable")]
    NotWritable = 9,

    #[error("Wrong account owner")]
    WrongOwner = 10,
}

impl From<PortalError> for ProgramError {
    fn from(e: PortalError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
