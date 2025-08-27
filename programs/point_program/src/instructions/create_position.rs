use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::{
    dlmm::{
        self, accounts::Position, cpi::{
            accounts::{ InitializePosition,AddLiquidityOneSide},
            add_liquidity_one_side, initialize_position,
        }, types::LiquidityOneSideParameter
    },
    error::ErrorCode,
    state::UserPoints,
};

#[derive(Accounts)]
pub struct CreatePosition<'info> {
    #[account(mut)]
    /// CHECK The pool account
    pub bin_array_bitmap_extension: Option<UncheckedAccount<'info>>,

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

    #[account(mut)]
    pub position: AccountLoader<'info, Position>,

    #[account(
        mut,
        seeds=[b"position_authority",user.key().as_ref()],
        bump
    )]
    /// CHECK Position authority
    pub position_authority: UncheckedAccount<'info>,

    #[account(
        mut,
        token::mint=usdc_mint
    )]
    pub user_usdc: Account<'info, TokenAccount>,
    pub usdc_mint: Account<'info, Mint>,

    #[account(mut)]
    /// CHECK The pool account
    pub lb_pair: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: Reserve account of usdc token
    pub reserve: UncheckedAccount<'info>,

    /// CHECK: DLMM bin array covering highest bin in user position
    pub bin_array_upper: UncheckedAccount<'info>,
    /// CHECK: DLMM bin array covering lowest bin in user position
    pub bin_array_lower: UncheckedAccount<'info>,

    #[account(address=dlmm::ID)]
    /// CHECK DLMM program
    pub dlmm_program: UncheckedAccount<'info>,

    /// CHECK: DLMM program event authority
    pub event_authority: UncheckedAccount<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn create_position_handler(
    ctx: Context<CreatePosition>,
    usdc_amount: u64,
    lower_bin_id: i32,
    width: i32,
    liquidity_parameter: LiquidityOneSideParameter,
) -> Result<()> {
    require!(usdc_amount > 0, ErrorCode::ZeroAmount);
    let user_point = &mut ctx.accounts.user_points;

    let user_key = ctx.accounts.user.key();
    let signer_seeds: &[&[&[u8]]] = &[&[
        b"position_authority",
        user_key.as_ref(),
        &[ctx.bumps.position_authority],
    ]];

    let initialize_position_accounts = InitializePosition {
        payer: ctx.accounts.user.to_account_info(),
        lb_pair: ctx.accounts.lb_pair.to_account_info(),
        position: ctx.accounts.position.to_account_info(),
        owner: ctx.accounts.position_authority.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
        event_authority: ctx.accounts.event_authority.to_account_info(),
        program: ctx.accounts.dlmm_program.to_account_info(),
    };
    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.dlmm_program.to_account_info(),
        initialize_position_accounts,
        signer_seeds,
    );
    initialize_position(cpi_context, lower_bin_id, width)?;

    let add_liquidity_accounts = AddLiquidityOneSide {
        position: ctx.accounts.position.to_account_info(),
        lb_pair: ctx.accounts.lb_pair.to_account_info(),
        bin_array_bitmap_extension: ctx
            .accounts
            .bin_array_bitmap_extension
            .as_ref()
            .map(|account| account.to_account_info()),
        user_token: ctx.accounts.user_usdc.to_account_info(),
        reserve: ctx.accounts.reserve.to_account_info(),
        token_mint: ctx.accounts.usdc_mint.to_account_info(),
        bin_array_lower: ctx.accounts.bin_array_lower.to_account_info(),
        bin_array_upper: ctx.accounts.bin_array_upper.to_account_info(),
        sender: ctx.accounts.user.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
        event_authority: ctx.accounts.event_authority.to_account_info(),
        program: ctx.accounts.dlmm_program.to_account_info(),
        
    };
    let cpi_context = CpiContext::new(
        ctx.accounts.dlmm_program.to_account_info(),
        add_liquidity_accounts,
    );
    add_liquidity_one_side(cpi_context,liquidity_parameter)?;

    user_point.points += usdc_amount;
    user_point.user = user_key;
    Ok(())
}
