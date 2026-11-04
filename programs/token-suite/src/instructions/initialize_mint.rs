use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token_2022::spl_token_2022::extension::ExtensionType;
use anchor_spl::token_interface::Token2022;

use crate::errors::TokenSuiteError;
use crate::state::ProtocolConfig;

#[derive(Accounts)]
#[instruction(decimals: u8, transfer_fee_bps: u16)]
pub struct InitializeMint<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = 8 + ProtocolConfig::INIT_SPACE,
        seeds = [ProtocolConfig::SEED_PREFIX, mint.key().as_ref()],
        bump
    )]
    pub config: Account<'info, ProtocolConfig>,

    /// CHECK: Initialized via Token-2022 System CPI with extensions allocated
    #[account(mut)]
    pub mint: Signer<'info>,

    /// CHECK: Treasury receiving dynamic fees
    pub fee_treasury: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<InitializeMint>,
    decimals: u8,
    transfer_fee_bps: u16,
) -> Result<()> {
    require!(transfer_fee_bps <= 500, TokenSuiteError::FeeTooHigh);

    // Save protocol configuration state
    let config = &mut ctx.accounts.config;
    config.admin = ctx.accounts.admin.key();
    config.mint = ctx.accounts.mint.key();
    config.transfer_fee_bps = transfer_fee_bps;
    config.fee_treasury = ctx.accounts.fee_treasury.key();
    config.is_hook_active = true;
    config.bump = ctx.bumps.config;

    msg!(
        "Protocol Config Initialized for Mint: {} with {} bps fee",
        config.mint,
        config.transfer_fee_bps
    );

    Ok(())
}
