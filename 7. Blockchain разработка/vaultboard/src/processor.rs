use crate::{
    error::VaultError,
    instruction::VaultInstruction,
    state::{MESSAGE_MAX, VaultState},
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    msg,
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
    system_instruction, system_program,
    sysvar::{Sysvar, rent::Rent},
};

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> solana_program::entrypoint::ProgramResult {
    let ix = VaultInstruction::try_from_slice(data).map_err(|_| VaultError::InvalidInstruction)?;

    match ix {
        VaultInstruction::Initialize { message } => {
            process_initialize(program_id, accounts, message)
        }
        VaultInstruction::Deposit { lamports } => process_deposit(program_id, accounts, lamports),
        VaultInstruction::UpdateMessage { message } => {
            process_update_message(program_id, accounts, message)
        }
        VaultInstruction::Withdraw { lamports } => process_withdraw(program_id, accounts, lamports),
    }
}

fn assert_system_program(acc: &AccountInfo) -> Result<(), VaultError> {
    if acc.key != &system_program::ID {
        return Err(VaultError::InvalidSystemProgram);
    }
    Ok(())
}

fn derive_vault_pda(program_id: &Pubkey, owner: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"vault", owner.as_ref()], program_id)
}

fn write_message(
    buf: &mut [u8; MESSAGE_MAX],
    len_dst: &mut u8,
    message: &str,
) -> Result<(), VaultError> {
    let bytes = message.as_bytes();
    if bytes.len() > MESSAGE_MAX {
        return Err(VaultError::MessageTooLong);
    }
    *len_dst = bytes.len() as u8;
    buf.fill(0);
    buf[..bytes.len()].copy_from_slice(bytes);
    Ok(())
}

/// Initialize { message }
/// Accounts:
/// 0) owner [signer, writable]
/// 1) vault_pda [writable]
/// 2) system_program []
fn process_initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    message: String,
) -> solana_program::entrypoint::ProgramResult {
    if accounts.len() != 3 {
        return Err(VaultError::WrongAccounts.into());
    }

    let acc_iter = &mut accounts.iter();
    let owner = next_account_info(acc_iter)?;
    let vault = next_account_info(acc_iter)?;
    let sys = next_account_info(acc_iter)?;

    assert_system_program(sys)?;

    if !owner.is_signer {
        return Err(VaultError::MissingSignature.into());
    }
    if !owner.is_writable || !vault.is_writable {
        return Err(VaultError::NotWritable.into());
    }

    let (expected_vault, bump) = derive_vault_pda(program_id, owner.key);
    if expected_vault != *vault.key {
        return Err(VaultError::InvalidVaultPda.into());
    }

    // Если уже существует — запрещаем повторную инициализацию
    if **vault.lamports.borrow() > 0 {
        return Err(VaultError::AlreadyInitialized.into());
    }

    let rent = Rent::get()?;
    let lamports = rent.minimum_balance(VaultState::LEN);

    let create_ix = system_instruction::create_account(
        owner.key,
        vault.key,
        lamports,
        VaultState::LEN as u64,
        program_id,
    );

    let seeds: &[&[u8]] = &[b"vault", owner.key.as_ref(), &[bump]];
    invoke_signed(
        &create_ix,
        &[owner.clone(), vault.clone(), sys.clone()],
        &[seeds],
    )?;

    let mut state = VaultState {
        is_initialized: true,
        owner: *owner.key,
        bump,
        message_len: 0,
        message: [0u8; MESSAGE_MAX],
        total_deposited: 0,
    };

    write_message(&mut state.message, &mut state.message_len, &message)?;
    state.serialize(&mut &mut vault.data.borrow_mut()[..])?;

    msg!("Vault initialized for owner {}", owner.key);
    Ok(())
}

/// Deposit { lamports }
/// Accounts:
/// 0) owner [signer, writable]  (from)
/// 1) vault_pda [writable]      (to)
/// 2) system_program []
fn process_deposit(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    lamports: u64,
) -> solana_program::entrypoint::ProgramResult {
    if accounts.len() != 3 {
        return Err(VaultError::WrongAccounts.into());
    }
    if lamports == 0 {
        return Err(VaultError::InvalidLamports.into());
    }

    let acc_iter = &mut accounts.iter();
    let owner = next_account_info(acc_iter)?;
    let vault = next_account_info(acc_iter)?;
    let sys = next_account_info(acc_iter)?;

    assert_system_program(sys)?;

    if !owner.is_signer {
        return Err(VaultError::MissingSignature.into());
    }
    if !owner.is_writable || !vault.is_writable {
        return Err(VaultError::NotWritable.into());
    }

    let (expected_vault, _bump) = derive_vault_pda(program_id, owner.key);
    if expected_vault != *vault.key {
        return Err(VaultError::InvalidVaultPda.into());
    }
    if vault.owner != program_id {
        return Err(VaultError::WrongVaultOwner.into());
    }

    let mut state = VaultState::try_from_slice(&vault.data.borrow())?;
    if !state.is_initialized {
        return Err(VaultError::NotInitialized.into());
    }
    if state.owner != *owner.key {
        return Err(VaultError::NotOwner.into());
    }

    let ix = system_instruction::transfer(owner.key, vault.key, lamports);
    invoke(&ix, &[owner.clone(), vault.clone(), sys.clone()])?;

    state.total_deposited = state.total_deposited.saturating_add(lamports);
    state.serialize(&mut &mut vault.data.borrow_mut()[..])?;

    msg!("Deposit: {} lamports", lamports);
    Ok(())
}

/// UpdateMessage { message }
/// Accounts:
/// 0) owner [signer]
/// 1) vault_pda [writable]
fn process_update_message(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    message: String,
) -> solana_program::entrypoint::ProgramResult {
    if accounts.len() != 2 {
        return Err(VaultError::WrongAccounts.into());
    }

    let acc_iter = &mut accounts.iter();
    let owner = next_account_info(acc_iter)?;
    let vault = next_account_info(acc_iter)?;

    if !owner.is_signer {
        return Err(VaultError::MissingSignature.into());
    }
    if !vault.is_writable {
        return Err(VaultError::NotWritable.into());
    }

    let (expected_vault, _bump) = derive_vault_pda(program_id, owner.key);
    if expected_vault != *vault.key {
        return Err(VaultError::InvalidVaultPda.into());
    }
    if vault.owner != program_id {
        return Err(VaultError::WrongVaultOwner.into());
    }

    let mut state = VaultState::try_from_slice(&vault.data.borrow())?;
    if !state.is_initialized {
        return Err(VaultError::NotInitialized.into());
    }
    if state.owner != *owner.key {
        return Err(VaultError::NotOwner.into());
    }

    write_message(&mut state.message, &mut state.message_len, &message)?;
    state.serialize(&mut &mut vault.data.borrow_mut()[..])?;

    msg!("Message updated");
    Ok(())
}

/// Withdraw { lamports }
/// Accounts:
/// 0) owner [signer, writable]  (to)
/// 1) vault_pda [writable]      (from)
/// 2) system_program []
fn process_withdraw(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    lamports: u64,
) -> solana_program::entrypoint::ProgramResult {
    if accounts.len() != 3 {
        return Err(VaultError::WrongAccounts.into());
    }
    if lamports == 0 {
        return Err(VaultError::InvalidLamports.into());
    }

    let acc_iter = &mut accounts.iter();
    let owner = next_account_info(acc_iter)?;
    let vault = next_account_info(acc_iter)?;
    let sys = next_account_info(acc_iter)?;

    assert_system_program(sys)?;

    if !owner.is_signer {
        return Err(VaultError::MissingSignature.into());
    }
    if !owner.is_writable || !vault.is_writable {
        return Err(VaultError::NotWritable.into());
    }

    let (expected_vault, _bump) = derive_vault_pda(program_id, owner.key);
    if expected_vault != *vault.key {
        return Err(VaultError::InvalidVaultPda.into());
    }
    if vault.owner != program_id {
        return Err(VaultError::WrongVaultOwner.into());
    }

    let state = VaultState::try_from_slice(&vault.data.borrow())?;
    if !state.is_initialized {
        return Err(VaultError::NotInitialized.into());
    }
    if state.owner != *owner.key {
        return Err(VaultError::NotOwner.into());
    }

    // Простое правило rent-exempt: после вывода vault должен оставаться >= rent_min
    let rent = Rent::get()?;
    let rent_min = rent.minimum_balance(VaultState::LEN);
    let vault_lamports = **vault.lamports.borrow();

    if vault_lamports.saturating_sub(lamports) < rent_min {
        return Err(VaultError::WouldBreakRentExempt.into());
    }

    let ix = system_instruction::transfer(vault.key, owner.key, lamports);
    let seeds: &[&[u8]] = &[b"vault", owner.key.as_ref(), &[state.bump]];

    invoke_signed(&ix, &[vault.clone(), owner.clone(), sys.clone()], &[seeds])?;

    msg!("Withdraw: {} lamports", lamports);
    Ok(())
}
