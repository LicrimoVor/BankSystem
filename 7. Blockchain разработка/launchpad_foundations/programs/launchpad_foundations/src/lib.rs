use anchor_lang::prelude::*;

declare_id!("Dyb1KLob1goNPmKmC9zdeu4eeg8xGZVqwA5x36xVHTsZ");

#[account]
pub struct LaunchpadConfig {
    pub admin: Pubkey,
    pub fee_usd: u64,
}

#[program]
pub mod launchpad_foundations {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.admin = ctx.accounts.signer.key();
        config.fee_usd = data;
        Ok(())
    }

    pub fn update(ctx: Context<UpdateFee>, data: u64) -> Result<()> {
        ctx.accounts.config.fee_usd = data;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(init, payer = signer, space = 8 + 32 + 8)]
    pub config: Account<'info, LaunchpadConfig>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateFee<'info> {
    pub admin: Signer<'info>,
    #[account(mut, has_one = admin)]
    pub config: Account<'info, LaunchpadConfig>,
}
