use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum VaultInstruction {
    Initialize { message: String },
    Deposit { lamports: u64 },
    UpdateMessage { message: String },
    Withdraw { lamports: u64 },
}
