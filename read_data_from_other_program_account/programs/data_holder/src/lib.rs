use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("2i5NAbznwnh3mjDk1gDUjTAGxXBykzXVk8Uvq2QyPH9i");

#[program]
pub mod data_holder {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.pda.x = 9;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = 8 + size_of::<MyData>(),
        seeds = [],
        bump
    )]
    pda: Account<'info, MyData>,
    system_program: Program<'info, System>,
}

#[account]
pub struct MyData {
    x: u64,
}
