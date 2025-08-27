#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

pub mod instructions;
use instructions::*;

pub mod error;
pub mod state;
declare_program!(dlmm);
use crate::dlmm::types::{BinLiquidityReduction, LiquidityOneSideParameter};

declare_id!("6uojdznPGFYevAMovWg3cdkmC8d7W1T3c69GRpwbTV2");

#[program]
pub mod point_program {

    use super::*;

    pub fn create_position(
        ctx: Context<CreatePosition>,
        usdc_amount: u64,
        lower_bin_id: i32,
        width: i32,
        liquidity_parameter: LiquidityOneSideParameter,
    ) -> Result<()> {
        create_position_handler(ctx, usdc_amount, lower_bin_id, width, liquidity_parameter)?;
        Ok(())
    }

    pub fn terminate_position(
        ctx: Context<TerminatePosition>,
        // bin_liquidity_removal: Vec<BinLiquidityReduction>,S
    ) -> Result<()> {
        terminate_position_handler(ctx,
            //  bin_liquidity_removalS
            )?;
        Ok(())
    }
}
