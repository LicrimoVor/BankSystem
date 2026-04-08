use crate::{error::PortalError, state::PortalState};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    msg,
    program::invoke_signed,
    pubkey::Pubkey,
    sysvar::{Sysvar, rent::Rent},
};
use solana_system_interface::instruction as system_instruction;

/// Теги инструкций (максимально просто: 1 байт).
/// 0 = Initialize, 1 = Open, 2 = Close
const IX_INIT: u8 = 0;
const IX_OPEN: u8 = 1;
const IX_CLOSE: u8 = 2;

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> solana_program::entrypoint::ProgramResult {
    let tag = data
        .first()
        .copied()
        .ok_or(PortalError::InvalidInstruction)?;

    match tag {
        IX_INIT => process_initialize(program_id, accounts),
        IX_OPEN => process_set_open(program_id, accounts, true),
        IX_CLOSE => process_set_open(program_id, accounts, false),
        _ => Err(PortalError::InvalidInstruction.into()),
    }
}

fn process_initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> solana_program::entrypoint::ProgramResult {
    let acc_iter = &mut accounts.iter();

    let owner = next_account_info(acc_iter)?; // игрок
    let portal_pda = next_account_info(acc_iter)?; // PDA двери
    let lever_program = next_account_info(acc_iter)?; // программа рычага
    let system_program = next_account_info(acc_iter)?; // system program

    if !owner.is_signer {
        return Err(PortalError::MissingSignature.into());
    }
    if !owner.is_writable {
        return Err(PortalError::NotWritable.into());
    }
    if !portal_pda.is_writable {
        return Err(PortalError::NotWritable.into());
    }
    if !lever_program.executable {
        return Err(PortalError::InvalidLeverProgram.into());
    }

    // PDA двери зависит от owner: ["portal", owner]
    let (expected_pda, bump) =
        Pubkey::find_program_address(&[b"portal", owner.key.as_ref()], program_id);
    if expected_pda != *portal_pda.key {
        return Err(PortalError::InvalidPortalPda.into());
    }

    // Если аккаунт уже существует — повторная инициализация запрещена.
    if **portal_pda.lamports.borrow() > 0 {
        msg!("portal_pda already exists");
        return Err(PortalError::AlreadyInitialized.into());
    }

    // Создаём PDA-аккаунт через System Program (нужен invoke_signed).
    let rent = Rent::get()?;
    let lamports = rent.minimum_balance(PortalState::LEN);

    let create_ix = system_instruction::create_account(
        owner.key,
        portal_pda.key,
        lamports,
        PortalState::LEN as u64,
        program_id,
    );

    let seeds: &[&[u8]] = &[b"portal", owner.key.as_ref(), &[bump]];

    invoke_signed(
        &create_ix,
        &[owner.clone(), portal_pda.clone(), system_program.clone()],
        &[seeds],
    )?;

    // Записываем начальное состояние.
    let state = PortalState {
        is_initialized: true,
        owner: *owner.key,
        lever_program: *lever_program.key,
        bump,
        is_open: 0,
    };

    state.serialize(&mut &mut portal_pda.data.borrow_mut()[..])?;

    msg!("Portal initialized. Door is CLOSED.");
    Ok(())
}

fn process_set_open(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    open: bool,
) -> solana_program::entrypoint::ProgramResult {
    let acc_iter = &mut accounts.iter();

    let player = next_account_info(acc_iter)?;
    let portal_pda = next_account_info(acc_iter)?;
    let lever_authority = next_account_info(acc_iter)?;
    let lever_program = next_account_info(acc_iter)?;

    if !player.is_signer {
        return Err(PortalError::MissingSignature.into());
    }
    if !portal_pda.is_writable {
        return Err(PortalError::NotWritable.into());
    }
    if portal_pda.owner != program_id {
        return Err(PortalError::WrongOwner.into());
    }

    // Ключевая проверка: lever_authority должен быть signer.
    if !lever_authority.is_signer {
        msg!("lever_authority is not signer => not called via Lever invoke_signed");
        return Err(PortalError::CheatingNotAllowed.into());
    }

    if !lever_program.executable {
        return Err(PortalError::InvalidLeverProgram.into());
    }

    // Читаем состояние двери.
    let mut state = PortalState::try_from_slice(&portal_pda.data.borrow())?;
    if !state.is_initialized {
        return Err(PortalError::NotInitialized.into());
    }
    if state.owner != *player.key {
        return Err(PortalError::NotOwner.into());
    }

    // Lever program должен совпасть с тем, что мы сохранили в Initialize.
    if state.lever_program != *lever_program.key {
        return Err(PortalError::InvalidLeverProgram.into());
    }

    // lever_authority должен быть PDA от lever_program с seed ["authority"].
    let (expected_auth, _auth_bump) =
        Pubkey::find_program_address(&[b"authority"], &state.lever_program);

    if expected_auth != *lever_authority.key {
        return Err(PortalError::InvalidLeverAuthority.into());
    }

    // Всё проверили — меняем состояние.
    state.is_open = if open { 1 } else { 0 };
    state.serialize(&mut &mut portal_pda.data.borrow_mut()[..])?;

    msg!("Door state changed. is_open = {}", state.is_open);
    Ok(())
}
