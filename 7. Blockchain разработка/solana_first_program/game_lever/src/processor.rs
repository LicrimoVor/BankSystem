use crate::error::LeverError;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    instruction::{AccountMeta, Instruction},
    msg,
    program::invoke_signed,
    pubkey::Pubkey,
};
use solana_system_interface::instruction as system_instruction;

/// 0 = InitAuthority, 1 = PressToOpen, 2 = PressToClose
const IX_INIT_AUTH: u8 = 0;
const IX_PRESS_OPEN: u8 = 1;
const IX_PRESS_CLOSE: u8 = 2;

/// Portal ожидает теги:
/// 0 = Initialize, 1 = Open, 2 = Close
const PORTAL_OPEN: u8 = 1;
const PORTAL_CLOSE: u8 = 2;

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> solana_program::entrypoint::ProgramResult {
    let tag = data
        .first()
        .copied()
        .ok_or(LeverError::InvalidInstruction)?;

    match tag {
        IX_INIT_AUTH => process_init_authority(program_id, accounts),
        IX_PRESS_OPEN => process_press(program_id, accounts, true),
        IX_PRESS_CLOSE => process_press(program_id, accounts, false),
        _ => Err(LeverError::InvalidInstruction.into()),
    }
}

/// Создаёт PDA lever_authority (чтобы он существовал как аккаунт).
///
/// Аккаунты:
/// 0. [signer, writable] payer (игрок)
/// 1. [writable] lever_authority_pda (PDA ["authority"])
/// 2. [] system_program
fn process_init_authority(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> solana_program::entrypoint::ProgramResult {
    let acc_iter = &mut accounts.iter();

    let payer = next_account_info(acc_iter)?;
    let lever_authority = next_account_info(acc_iter)?;
    let system_program = next_account_info(acc_iter)?;

    if !payer.is_signer {
        return Err(LeverError::MissingSignature.into());
    }
    if !payer.is_writable || !lever_authority.is_writable {
        return Err(LeverError::NotWritable.into());
    }

    let (expected_auth, bump) = Pubkey::find_program_address(&[b"authority"], program_id);
    if expected_auth != *lever_authority.key {
        return Err(LeverError::InvalidAuthorityPda.into());
    }

    // Если PDA уже создан (имеет lamports), можно ничего не делать.
    if **lever_authority.lamports.borrow() > 0 {
        msg!("lever_authority already exists");
        return Ok(());
    }

    // Создаём аккаунт с 0 data. Для простоты кладём 1 lamport.
    let create_ix = system_instruction::create_account(
        payer.key,
        lever_authority.key,
        1,          // минимально, лишь бы аккаунт существовал
        0,          // space = 0
        program_id, // owner = Lever program
    );

    let seeds: &[&[u8]] = &[b"authority", &[bump]];

    invoke_signed(
        &create_ix,
        &[
            payer.clone(),
            lever_authority.clone(),
            system_program.clone(),
        ],
        &[seeds],
    )?;

    msg!("lever_authority created");
    Ok(())
}

/// Нажатие рычага: делает CPI в Portal (Open/Close).
///
/// Аккаунты (важен порядок, потому что Portal ожидает фиксированный порядок):
/// 0. [signer] player
/// 1. [writable] portal_pda
/// 2. [] portal_program (исполняемый аккаунт программы Portal)
/// 3. [] lever_authority_pda (PDA ["authority"], станет signer внутри CPI)
/// 4. [] lever_program (исполняемый аккаунт ЭТОЙ программы; Portal хранит его pubkey в state)
fn process_press(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    open: bool,
) -> solana_program::entrypoint::ProgramResult {
    let acc_iter = &mut accounts.iter();

    let player = next_account_info(acc_iter)?;
    let portal_pda = next_account_info(acc_iter)?;
    let portal_program = next_account_info(acc_iter)?;
    let lever_authority = next_account_info(acc_iter)?;
    let lever_program_account = next_account_info(acc_iter)?;

    if !player.is_signer {
        return Err(LeverError::MissingSignature.into());
    }
    if !portal_pda.is_writable {
        // Portal будет менять state, значит, аккаунт должен быть writable.
        return Err(LeverError::NotWritable.into());
    }
    if !portal_program.executable {
        // Мы хотим вызывать именно программу Portal.
        return Err(LeverError::InvalidInstruction.into());
    }

    // Проверяем, что "lever_program" аккаунт действительно соответствует нашему program_id.
    // Это нужно, потому что Portal сверяет lever_program из state с переданным аккаунтом.
    if !lever_program_account.executable || *lever_program_account.key != *program_id {
        return Err(LeverError::InvalidLeverProgramAccount.into());
    }

    // Проверяем PDA lever_authority.
    let (expected_auth, bump) = Pubkey::find_program_address(&[b"authority"], program_id);
    if expected_auth != *lever_authority.key {
        return Err(LeverError::InvalidAuthorityPda.into());
    }

    // Готовим инструкцию для Portal.
    // Portal ждёт data = [1] для Open или [2] для Close.
    let portal_tag = if open { PORTAL_OPEN } else { PORTAL_CLOSE };

    let ix = Instruction {
        program_id: *portal_program.key,
        accounts: vec![
            // Portal: player signer (readonly достаточно)
            AccountMeta::new_readonly(*player.key, true),
            // Portal: portal_pda writable (state)
            AccountMeta::new(*portal_pda.key, false),
            // Portal: lever_authority signer (readonly)
            AccountMeta::new_readonly(*lever_authority.key, true),
            // Portal: lever_program (readonly, executable)
            AccountMeta::new_readonly(*lever_program_account.key, false),
        ],
        data: vec![portal_tag],
    };

    // Самое важное: CPI через invoke_signed.
    // Мы "подписываем" lever_authority PDA, поэтому внутри Portal
    // lever_authority будет signer=true.
    let seeds: &[&[u8]] = &[b"authority", &[bump]];

    invoke_signed(
        &ix,
        &[
            player.clone(),
            portal_pda.clone(),
            lever_authority.clone(),
            lever_program_account.clone(),
            portal_program.clone(), // account программы, как в примерах system_program
        ],
        &[seeds],
    )?;

    msg!("Lever pressed. Requested open = {}", open);
    Ok(())
}
