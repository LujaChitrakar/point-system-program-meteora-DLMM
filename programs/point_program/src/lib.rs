#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

pub mod instructions;
use instructions::*;

pub mod state;
declare_program!(dlmm);
use crate::dlmm::*;

declare_id!("6uojdznPGFYevAMovWg3cdkmC8d7W1T3c69GRpwbTV2");

#[program]
pub mod point_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
