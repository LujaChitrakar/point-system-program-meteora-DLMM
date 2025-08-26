use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The amount is Zero")]
    ZeroAmount,
    #[msg("Invalid User")]
    InvalidUser,
}
