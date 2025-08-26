
use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};

use crate::{dlmm::{self, accounts::Position, program::LbClmm}, state::UserPoints};

#[derive(Accounts)]
pub struct CreatePosition<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer=user,
        space=8+UserPoints::INIT_SPACE,
        seeds=[b"user_position",user.key().as_ref()],
        bump
    )]
    pub user_points: Account<'info, UserPoints>,

    #[account(
        mut
    )]
    pub position:AccountLoader<'info,Position>,

    #[account(
        init_if_needed,
        payer=user,
        associated_token::mint=usdc_mint,
        associated_token::authority=position_authority
    )]
    pub position_usdc:Account<'info,TokenAccount>,

    #[account(
        mut,
        seeds=[b"position_authority",user.key().as_ref()],
        bump
    )]
    /// CHECK Position authority 
    pub position_authority:UncheckedAccount<'info>,

    #[account(mut)]
    pub user_usdc:Account<'info,TokenAccount>,

    pub usdc_mint:Account<'info,Mint>,  

    #[account(address=dlmm::ID)]
    /// CHECK DLMM program 
    pub dlmm_program:UncheckedAccount<'info>,

    /// CHECK: DLMM program event authority 
    pub event_authority:UncheckedAccount<'info>,

    pub associated_token_program:Program<'info,AssociatedToken>,
    pub token_program:Program<'info,Token>,
    pub system_program:Program<'info,System>
}

pub fn create_position_handler(ctx:Context<CreatePosition>)->Result<()>{
    
}