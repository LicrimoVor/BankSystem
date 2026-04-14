use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount};

declare_id!("9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP");

pub const EXPECTED_DECIMALS: u8 = 6;
pub const MAX_STALENESS_SLOTS: u64 = 100;
pub const LAMPORTS_PER_SOL_U64: u64 = 1_000_000_000;

#[program]
pub mod lesson_performance {
    use super::*;

    pub fn initialize_oracle(ctx: Context<InitializeOracle>, initial_price: u64) -> Result<()> {
        require!(initial_price > 0, PerformanceError::InvalidPrice);
        let oracle = &mut ctx.accounts.oracle;
        oracle.admin = ctx.accounts.admin.key();
        oracle.price = initial_price;
        oracle.decimals = EXPECTED_DECIMALS;
        oracle.last_updated_slot = Clock::get()?.slot;
        oracle.bump = ctx.bumps.oracle;
        Ok(())
    }

    pub fn update_price(ctx: Context<UpdateOracle>, new_price: u64) -> Result<()> {
        require!(new_price > 0, PerformanceError::InvalidPrice);
        let oracle = &mut ctx.accounts.oracle;
        oracle.price = new_price;
        oracle.last_updated_slot = Clock::get()?.slot;
        Ok(())
    }

    pub fn set_oracle_last_updated_slot(ctx: Context<UpdateOracle>, slot: u64) -> Result<()> {
        ctx.accounts.oracle.last_updated_slot = slot;
        Ok(())
    }

    pub fn create_token_with_fee_baseline(
        ctx: Context<CreateTokenWithFeeBaseline>,
        initial_supply: u64,
        fee_usd: u64,
    ) -> Result<()> {
        let (fee_lamports, minted_raw) = process_fee_and_mint(
            &ctx.accounts.payer,
            &ctx.accounts.treasury,
            &ctx.accounts.oracle,
            &ctx.accounts.mint,
            &ctx.accounts.payer_ata,
            &ctx.accounts.mint_authority,
            &ctx.accounts.token_program,
            &ctx.accounts.system_program,
            ctx.bumps.mint_authority,
            initial_supply,
            fee_usd,
        )?;

        let _unused = (
            ctx.accounts.oracle_config.key(),
            ctx.accounts.metadata_program.key(),
        );

        emit_sample(0, fee_lamports, minted_raw)?;
        Ok(())
    }

    pub fn create_token_with_fee_optimized(
        ctx: Context<CreateTokenWithFeeOptimized>,
        initial_supply: u64,
        fee_usd: u64,
    ) -> Result<()> {
        let (fee_lamports, minted_raw) = process_fee_and_mint(
            &ctx.accounts.payer,
            &ctx.accounts.treasury,
            &ctx.accounts.oracle,
            &ctx.accounts.mint,
            &ctx.accounts.payer_ata,
            &ctx.accounts.mint_authority,
            &ctx.accounts.token_program,
            &ctx.accounts.system_program,
            ctx.bumps.mint_authority,
            initial_supply,
            fee_usd,
        )?;

        emit_sample(1, fee_lamports, minted_raw)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeOracle<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + 32 + 8 + 1 + 8 + 1,
        seeds = [b"oracle"],
        bump
    )]
    pub oracle: Account<'info, OracleState>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateOracle<'info> {
    #[account(
        mut,
        seeds = [b"oracle"],
        bump = oracle.bump,
        has_one = admin
    )]
    pub oracle: Account<'info, OracleState>,

    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct CreateTokenWithFeeBaseline<'info> {
    #[account(
        init,
        payer = payer,
        mint::decimals = EXPECTED_DECIMALS,
        mint::authority = mint_authority,
        mint::freeze_authority = mint_authority
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer
    )]
    pub payer_ata: Account<'info, TokenAccount>,

    #[account(
        seeds = [b"mint_authority", mint.key().as_ref()],
        bump
    )]
    /// CHECK: PDA signer for mint authority, verified by seeds.
    pub mint_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub treasury: SystemAccount<'info>,

    #[account(seeds = [b"oracle"], bump = oracle.bump)]
    pub oracle: Account<'info, OracleState>,

    /// CHECK: deliberately unused baseline account for CU profiling.
    pub oracle_config: UncheckedAccount<'info>,

    /// CHECK: deliberately unused baseline account for CU profiling.
    pub metadata_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct CreateTokenWithFeeOptimized<'info> {
    #[account(
        init,
        payer = payer,
        mint::decimals = EXPECTED_DECIMALS,
        mint::authority = mint_authority,
        mint::freeze_authority = mint_authority
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer
    )]
    pub payer_ata: Account<'info, TokenAccount>,

    #[account(
        seeds = [b"mint_authority", mint.key().as_ref()],
        bump
    )]
    /// CHECK: PDA signer for mint authority, verified by seeds.
    pub mint_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub treasury: SystemAccount<'info>,

    #[account(seeds = [b"oracle"], bump = oracle.bump)]
    pub oracle: Account<'info, OracleState>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
pub struct OracleState {
    pub admin: Pubkey,
    pub price: u64,
    pub decimals: u8,
    pub last_updated_slot: u64,
    pub bump: u8,
}

#[event]
pub struct FeePathExecuted {
    pub path: u8,
    pub fee_lamports: u64,
    pub minted_raw: u64,
    pub slot: u64,
}

#[error_code]
pub enum PerformanceError {
    #[msg("Invalid oracle price")]
    InvalidPrice,
    #[msg("Bad oracle decimals")]
    BadOracleDecimals,
    #[msg("Stale oracle data")]
    StaleOracle,
    #[msg("Math overflow")]
    MathOverflow,
}

fn emit_sample(path: u8, fee_lamports: u64, minted_raw: u64) -> Result<()> {
    let clock = Clock::get()?;
    emit!(FeePathExecuted {
        path,
        fee_lamports,
        minted_raw,
        slot: clock.slot,
    });
    Ok(())
}

fn process_fee_and_mint<'info>(
    payer: &Signer<'info>,
    treasury: &SystemAccount<'info>,
    oracle: &Account<'info, OracleState>,
    mint: &Account<'info, Mint>,
    destination: &Account<'info, TokenAccount>,
    mint_authority: &UncheckedAccount<'info>,
    token_program: &Program<'info, Token>,
    system_program: &Program<'info, System>,
    mint_authority_bump: u8,
    initial_supply: u64,
    fee_usd: u64,
) -> Result<(u64, u64)> {
    validate_oracle(oracle)?;
    require_fresh(oracle)?;

    let fee_lamports = calc_fee_lamports(fee_usd, oracle.price)?;
    let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
        &payer.key(),
        &treasury.key(),
        fee_lamports,
    );
    anchor_lang::solana_program::program::invoke(
        &transfer_ix,
        &[
            payer.to_account_info(),
            treasury.to_account_info(),
            system_program.to_account_info(),
        ],
    )?;

    let minted_raw = calc_amount_raw(initial_supply)?;
    let mint_key = mint.key();
    let signer_seeds: &[&[u8]] = &[
        b"mint_authority",
        mint_key.as_ref(),
        &[mint_authority_bump],
    ];
    let signer_seeds_arr = [signer_seeds];
    let cpi_accounts = MintTo {
        mint: mint.to_account_info(),
        to: destination.to_account_info(),
        authority: mint_authority.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(
        token_program.to_account_info(),
        cpi_accounts,
        &signer_seeds_arr,
    );
    token::mint_to(cpi_ctx, minted_raw)?;

    Ok((fee_lamports, minted_raw))
}

fn calc_amount_raw(initial_supply: u64) -> Result<u64> {
    let multiplier = 10u64
        .checked_pow(EXPECTED_DECIMALS as u32)
        .ok_or(PerformanceError::MathOverflow)?;
    let amount_raw = initial_supply
        .checked_mul(multiplier)
        .ok_or(PerformanceError::MathOverflow)?;
    Ok(amount_raw)
}

fn calc_fee_lamports(fee_usd: u64, price: u64) -> Result<u64> {
    require!(price > 0, PerformanceError::InvalidPrice);

    let fee_u128 = fee_usd as u128;
    let price_u128 = price as u128;
    let lps_u128 = LAMPORTS_PER_SOL_U64 as u128;

    let numerator = fee_u128
        .checked_mul(lps_u128)
        .ok_or(PerformanceError::MathOverflow)?;
    let fee_lamports_u128 = numerator
        .checked_div(price_u128)
        .ok_or(PerformanceError::MathOverflow)?;
    let fee_lamports =
        u64::try_from(fee_lamports_u128).map_err(|_| PerformanceError::MathOverflow)?;

    Ok(fee_lamports)
}

fn validate_oracle(oracle: &OracleState) -> Result<()> {
    require!(
        oracle.decimals == EXPECTED_DECIMALS,
        PerformanceError::BadOracleDecimals
    );
    require!(oracle.price > 0, PerformanceError::InvalidPrice);
    Ok(())
}

fn require_fresh(oracle: &OracleState) -> Result<()> {
    let clock = Clock::get()?;
    let slots_ago = clock.slot.saturating_sub(oracle.last_updated_slot);
    require!(slots_ago <= MAX_STALENESS_SLOTS, PerformanceError::StaleOracle);
    Ok(())
}
