use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ProtocolConfig {
    /// Authority allowed to update protocol configurations
    pub admin: Pubkey,
    /// Associated Token-2022 mint address
    pub mint: Pubkey,
    /// Transfer fee in basis points (e.g. 50 = 0.50%)
    pub transfer_fee_bps: u16,
    /// Fee recipient account
    pub fee_treasury: Pubkey,
    /// Flag indicating whether transfer hook enforcement is active
    pub is_hook_active: bool,
    /// Bump seed for PDA derivation
    pub bump: u8,
}

impl ProtocolConfig {
    pub const SEED_PREFIX: &'static [u8] = b"protocol-config";
}
