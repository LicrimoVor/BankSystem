use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum VaultError {
    #[error("Invalid instruction data")]
    InvalidInstruction = 0,

    #[error("Wrong number of accounts")]
    WrongAccounts = 1,

    #[error("Missing required signature")]
    MissingSignature = 2,

    #[error("Account must be writable")]
    NotWritable = 3,

    #[error("Invalid vault PDA")]
    InvalidVaultPda = 4,

    #[error("Vault account is not owned by this program")]
    WrongVaultOwner = 5,

    #[error("Invalid system program account")]
    InvalidSystemProgram = 6,

    #[error("Already initialized")]
    AlreadyInitialized = 7,

    #[error("Not initialized")]
    NotInitialized = 8,

    #[error("Not the vault owner")]
    NotOwner = 9,

    #[error("Message too long (max 64 bytes)")]
    MessageTooLong = 10,

    #[error("Lamports must be > 0")]
    InvalidLamports = 11,

    #[error("Would break rent-exempt minimum")]
    WouldBreakRentExempt = 12,
}

impl From<VaultError> for ProgramError {
    fn from(e: VaultError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
