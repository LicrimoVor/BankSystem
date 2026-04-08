use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

pub const MESSAGE_MAX: usize = 64;

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct VaultState {
    pub is_initialized: bool,
    pub owner: Pubkey,
    pub bump: u8,
    pub message_len: u8,
    pub message: [u8; MESSAGE_MAX],
    pub total_deposited: u64,
}

impl VaultState {
    pub const LEN: usize = 1 + 32 + 1 + 1 + MESSAGE_MAX + 8;
}
