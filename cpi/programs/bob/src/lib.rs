use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("9UN4Kvc3cBg5FrHP33Lj4s9zXUZyDxUrnetaZ4NUYFnj");

#[program]
pub mod bob {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn set(ctx: Context<Set>, d: u64) -> Result<()> {
        ctx.accounts.bob_account.data = d;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set<'info> {
    #[account(mut)]
     bob_account: Account<'info, BobData>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + size_of::<BobData>(),
    )]
    bob_account: Account<'info, BobData>,

    system_program: Program<'info, System>,
}

#[account]
pub struct BobData {
    pub data: u64,
}
