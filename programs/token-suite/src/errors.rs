use anchor_lang::prelude::*;

#[error_code]
pub enum TokenSuiteError {
    #[msg("Transfer amount is below minimum permitted threshold")]
    AmountTooLow,

    #[msg("Account has been blacklisted by protocol governance")]
    AccountBlacklisted,

    #[msg("Transfer hook validation failed: signature missing or invalid")]
    InvalidHookSignature,

    #[msg("Dynamic fee exceeds maximum allowed percentage (max 500 bps)")]
    FeeTooHigh,

    #[msg("Token-2022 extension metadata exceeds allotted buffer")]
    MetadataBufferOverflow,

    #[msg("Unauthorized protocol administrator authority")]
    UnauthorizedAdmin,
}
