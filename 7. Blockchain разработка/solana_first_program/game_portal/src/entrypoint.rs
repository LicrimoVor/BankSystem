use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

entrypoint!(process_instruction);

/// Точка входа Solana-программы.
/// Валидатор вызывает её, когда транзакция адресована вашему program_id.
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    crate::processor::process(program_id, accounts, data)
}
