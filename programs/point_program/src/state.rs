use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserPoints {
    pub user: Pubkey,
    pub points: u64,
}

#[account]
#[derive(InitSpace)]
pub struct Position {
    pub owner: Pubkey,
    pub usdc_amount: u64,
}
