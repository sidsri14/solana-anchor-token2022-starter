use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("Tokn221111111111111111111111111111111111111");

#[program]
pub mod token_suite {
    use super::*;

    /// Initializes a new Token-2022 mint and stores the protocol configuration PDA
    pub fn initialize_mint(
        ctx: Context<InitializeMint>,
        decimals: u8,
        transfer_fee_bps: u16,
    ) -> Result<()> {
        instructions::initialize_mint::handler(ctx, decimals, transfer_fee_bps)
    }

    /// Mints tokens to a recipient Token-2022 ATA
    pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        instructions::mint_tokens::handler(ctx, amount)
    }

    /// SPL Transfer Hook Interface execution handler
    /// Automatically invoked by Token-2022 during every transfer CPI
    pub fn transfer_hook(ctx: Context<ExecuteTransferHook>, amount: u64) -> Result<()> {
        instructions::transfer_hook::handler(ctx, amount)
    }
}
