use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

use crate::{
    dlmm::{
        self,
        accounts::Position,
        cpi::{
            accounts::{ClosePosition, RemoveLiquidity},
            close_position, remove_liquidity,
        },
        types::BinLiquidityReduction,
    },
    error::ErrorCode,
    state::UserPoints,
};

#[derive(Accounts)]
pub struct TerminatePosition<'info> {
    #[account(mut)]
    /// CHECK The pool account
    pub bin_array_bitmap_extension: Option<UncheckedAccount<'info>>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds=[b"user_position",user.key().as_ref()],
        bump
    )]
    pub user_points: Account<'info, UserPoints>,

    #[account(mut)]
    pub position: AccountLoader<'info, Position>,

    // #[account(
    //     mut,
    //     associated_token::mint=usdc_mint,
    //     associated_token::authority=position_authority
    // )]
    // pub position_usdc: Account<'info, TokenAccount>,
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
}

pub fn terminate_position_handler(
    ctx: Context<TerminatePosition>,
    bin_liquidity_removal: Vec<BinLiquidityReduction>,
) -> Result<()> {
    let user_points = &mut ctx.accounts.user_points;

    require!(
        user_points.user == ctx.accounts.user.key(),
        ErrorCode::InvalidUser
    );

    let position_data = ctx.accounts.position.load()?;
    require!(
        position_data.owner == ctx.accounts.position_authority.key(),
        ErrorCode::InvalidOwner
    );

    let user_key = ctx.accounts.user.key();
    let signer_seeds: &[&[&[u8]]] = &[&[
        b"position_authority",
        user_key.as_ref(),
        &[ctx.bumps.position_authority],
    ]];

    let remove_liquidity_accounts = RemoveLiquidity {
        position: ctx.accounts.position.to_account_info(),
        lb_pair: ctx.accounts.lb_pair.to_account_info(),
        bin_array_bitmap_extension: ctx
            .accounts
            .bin_array_bitmap_extension
            .as_ref()
            .map(|account| account.to_account_info()),
        user_token_x: ctx.accounts.user_usdc.to_account_info(),
        user_token_y: ctx.accounts.user_usdc.to_account_info(),
        reserve_x: ctx.accounts.lb_pair.to_account_info(),
        reserve_y: ctx.accounts.lb_pair.to_account_info(),
        token_x_mint: ctx.accounts.usdc_mint.to_account_info(),
        token_y_mint: ctx.accounts.usdc_mint.to_account_info(),
        bin_array_lower: ctx.accounts.bin_array_lower.to_account_info(),
        bin_array_upper: ctx.accounts.bin_array_upper.to_account_info(),
        sender: ctx.accounts.position_authority.to_account_info(),
        token_x_program: ctx.accounts.token_program.to_account_info(),
        token_y_program: ctx.accounts.token_program.to_account_info(),
        event_authority: ctx.accounts.event_authority.to_account_info(),
        program: ctx.accounts.dlmm_program.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.dlmm_program.to_account_info(),
        remove_liquidity_accounts,
        signer_seeds,
    );
    remove_liquidity(cpi_ctx, bin_liquidity_removal)?;

    let close_position_accounts = ClosePosition {
        position: ctx.accounts.position.to_account_info(),
        lb_pair: ctx.accounts.lb_pair.to_account_info(),
        bin_array_lower: ctx.accounts.bin_array_lower.to_account_info(),
        bin_array_upper: ctx.accounts.bin_array_upper.to_account_info(),
        sender: ctx.accounts.position_authority.to_account_info(),
        rent_receiver: ctx.accounts.user.to_account_info(),
        event_authority: ctx.accounts.event_authority.to_account_info(),
        program: ctx.accounts.dlmm_program.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.dlmm_program.to_account_info(),
        close_position_accounts,
        signer_seeds,
    );
    close_position(cpi_ctx)?;

    // let total_balance = ctx.accounts.user_usdc.amount;
    // let cpi_ctx_transfer = CpiContext::new_with_signer(
    //     ctx.accounts.token_program.to_account_info(),
    //     Transfer {
    //         from: ctx.accounts.position_usdc.to_account_info(),
    //         to: ctx.accounts.user_usdc.to_account_info(),
    //         authority: ctx.accounts.position_authority.to_account_info(),
    //     },
    //     signer_seeds,
    // );
    // transfer(cpi_ctx_transfer, total_balance)?;

    user_points.points = 0;

    Ok(())
}
