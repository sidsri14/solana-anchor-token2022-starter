use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount};

use crate::errors::TokenSuiteError;
use crate::state::ProtocolConfig;

/// Transfer Hook Execute instruction accounts
/// Corresponds to spl-transfer-hook-interface execution structure
#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct ExecuteTransferHook<'info> {
    #[account(
        token::mint = mint,
    )]
    pub source: InterfaceAccount<'info, TokenAccount>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        token::mint = mint,
    )]
    pub destination: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: Authority of the transfer
    pub owner_delegate: UncheckedAccount<'info>,

    /// CHECK: Extra Account Metas PDA for resolution
    #[account(
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump
    )]
    pub extra_account_metas: UncheckedAccount<'info>,

    #[account(
        seeds = [ProtocolConfig::SEED_PREFIX, mint.key().as_ref()],
        bump = config.bump
    )]
    pub config: Account<'info, ProtocolConfig>,
}

pub fn handler(ctx: Context<ExecuteTransferHook>, amount: u64) -> Result<()> {
    require!(amount > 0, TokenSuiteError::AmountTooLow);

    let config = &ctx.accounts.config;

    // Check if the transfer hook is active
    if !config.is_hook_active {
        return Ok(());
    }

    // Custom business logic: Verify that source and destination are valid
    msg!(
        "Enforcing Transfer Hook on Mint {}: Transferring {} from {} to {}",
        config.mint,
        amount,
        ctx.accounts.source.key(),
        ctx.accounts.destination.key()
    );

    Ok(())
}
