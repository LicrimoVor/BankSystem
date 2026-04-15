use anchor_lang::prelude::*;
pub mod error;
pub mod state;
use crate::error::OracleError;
pub use state::OracleState;

pub const PRICE_DECIMALS: u8 = 6;
declare_id!("4cuvLFFqhaKnTHfeq2FtTUvgudRSe7wq982fA9PBUqBU");

fn apply_price_update(oracle: &mut OracleState, new_price: u64, current_slot: u64) -> Result<()> {
    // require!(
    //     oracle.last_updated_slot < current_slot,
    //     OracleError::InvalidSlot
    // );

    // let min = (oracle.price as u128).mul(8).div(10);
    // let max = (oracle.price as u128).mul(12).div(10);

    // require!(
    //     new_price as u128 >= min && new_price as u128 <= max,
    //     OracleError::InvalidPrice
    // );
    oracle.last_updated_slot = current_slot;
    oracle.price = new_price;
    Ok(())
}

#[program]
pub mod sol_usd_oracle {
    use super::*;

    pub fn initialize_oracle(ctx: Context<InitializeOracle>, admin: Pubkey) -> Result<()> {
        let oracle = &mut ctx.accounts.oracle;
        oracle.admin = admin;
        oracle.price = 0;
        oracle.decimals = PRICE_DECIMALS;
        oracle.last_updated_slot = Clock::get()?.slot;
        oracle.bump = ctx.bumps.oracle;
        Ok(())
    }

    pub fn update_price(ctx: Context<UpdatePrice>, new_price: u64) -> Result<()> {
        require!(new_price > 0, OracleError::InvalidPrice);

        let oracle = &mut ctx.accounts.oracle;
        require_keys_eq!(
            ctx.accounts.admin.key(),
            oracle.admin,
            OracleError::Unauthorized
        );

        let current_slot = Clock::get()?.slot;
        apply_price_update(oracle, new_price, current_slot)
    }
}

#[derive(Accounts)]
pub struct InitializeOracle<'info> {
    #[account(
        init,
        payer = payer,
        seeds = [OracleState::SEED],
        bump,
        space = 8 + OracleState::SIZE
    )]
    pub oracle: Account<'info, OracleState>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdatePrice<'info> {
    #[account(mut, seeds = [OracleState::SEED], bump = oracle.bump, has_one = admin)]
    pub oracle: Account<'info, OracleState>,
    pub admin: Signer<'info>,
}
