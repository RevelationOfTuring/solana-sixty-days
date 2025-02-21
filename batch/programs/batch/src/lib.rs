use anchor_lang::prelude::*;

declare_id!("DowHyddzxeixJSyxXjTx2Gjctgg9NLC7bbqA3nuqMwxp");

#[program]
pub mod batch {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn set(ctx: Context<Set>, new_val: u32) -> Result<()> {
        ctx.accounts.pda.value = new_val;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set<'info> {
    #[account(mut)]
    pda: Account<'info, Pda>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = std::mem::size_of::<Pda>() + 8,
    )]
    pda: Account<'info, Pda>,
    system_program: Program<'info, System>,
}

#[account]
pub struct Pda {
    value: u32,
}
