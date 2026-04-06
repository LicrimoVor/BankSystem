use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Осторожно: логирование Pubkey может быть заметным по Compute Units (CU) в mainnet, используйте только для дебага
    msg!("hello_program: invoked");
    msg!("hello_program: program_id = {}", program_id);
    msg!("hello_program: accounts_len = {}", accounts.len());
    msg!(
        "hello_program: instruction_data_len = {}",
        instruction_data.len()
    );

    // Для наглядности: если передали данные, распечатаем первый байт
    if let Some(first) = instruction_data.first() {
        msg!("hello_program: first_byte = {}", first);
    } else {
        msg!("hello_program: no data");
    }

    Ok(())
}
