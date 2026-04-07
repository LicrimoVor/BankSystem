use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

const COUNTER_SIZE: usize = 8;

#[repr(u8)]
pub enum CounterIx {
    Initialize = 0,
    Increment = 1,
}

impl CounterIx {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let tag = *input.get(0).ok_or(ProgramError::InvalidInstructionData)?;
        match tag {
            0 => Ok(CounterIx::Initialize),
            1 => Ok(CounterIx::Increment),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let ix = CounterIx::unpack(instruction_data)?;

    let account_info_iter = &mut accounts.iter();

    // Первый аккаунт — data account (counter state)
    let counter_acc = next_account_info(account_info_iter)?;

    // Проверки “как в реальной жизни”
    if counter_acc.owner != program_id {
        msg!("Counter account owner mismatch");
        return Err(ProgramError::IncorrectProgramId);
    }
    if !counter_acc.is_writable {
        msg!("Counter account must be writable");
        return Err(ProgramError::InvalidAccountData);
    }
    if counter_acc.data_len() != COUNTER_SIZE {
        msg!("Counter account must have exactly 8 bytes of data");
        return Err(ProgramError::InvalidAccountData);
    }

    let mut data = counter_acc.try_borrow_mut_data()?;

    match ix {
        CounterIx::Initialize => {
            // Явная инициализация: записываем 0
            data.copy_from_slice(&0u64.to_le_bytes());
            msg!("Initialized counter to 0");
        }
        CounterIx::Increment => {
            let current = u64::from_le_bytes(data[..8].try_into().unwrap());
            let next = current
                .checked_add(1)
                .ok_or(ProgramError::InvalidInstructionData)?;
            data.copy_from_slice(&next.to_le_bytes());
            msg!("Incremented: {} -> {}", current, next);
        }
    }

    Ok(())
}
