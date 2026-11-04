use anchor_lang::prelude::*;
use anchor_spl::token_2022::{self, MintTo};
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};

use crate::errors::TokenSuiteError;
use crate::state::ProtocolConfig;

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(
        seeds = [ProtocolConfig::SEED_PREFIX, mint.key().as_ref()],
        bump = config.bump,
        has_one = admin @ TokenSuiteError::UnauthorizedAdmin,
        has_one = mint
    )]
    pub config: Account<'info, ProtocolConfig>,

    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub recipient: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Program<'info, Token2022>,
}

pub fn handler(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
    require!(amount > 0, TokenSuiteError::AmountTooLow);

    let seeds = &[
        ProtocolConfig::SEED_PREFIX,
        ctx.accounts.mint.to_account_info().key.as_ref(),
        &[ctx.accounts.config.bump],
    ];
    let signer = &[&seeds[..]];

    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.recipient.to_account_info(),
        authority: ctx.accounts.config.to_account_info(),
    };

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer,
    );

    token_2022::mint_to(cpi_ctx, amount)?;

    msg!("Successfully minted {} tokens to {}", amount, ctx.accounts.recipient.key());
    Ok(())
}
