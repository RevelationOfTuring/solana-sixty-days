use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("2vir3aa6Haf2rje5GRuyAZBzL7TdZy3nTNPdBuMWJMU5");

#[program]
pub mod close {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn close(_ctx: Context<Close>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer = signer, 
        space = size_of::<Pda>() + 8, 
        seeds = [], 
        bump
    )]
    pub pda: Account<'info, Pda>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        mut,
        close = signer,
    )]
    pda: Account<'info, Pda>,
}

#[account]
pub struct Pda {
    pub x: u32,
}
