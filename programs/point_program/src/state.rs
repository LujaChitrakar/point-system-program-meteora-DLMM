use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserPoints {
    pub user: Pubkey,
    pub points: u64,
}
