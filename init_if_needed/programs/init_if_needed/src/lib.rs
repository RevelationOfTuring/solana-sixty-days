use anchor_lang::prelude::*;

declare_id!("njrYe3M7ZK3mgbHxtSmE7TeDVY69ELJbsQCWDiaFWyJ");

#[program]
pub mod init_if_needed {
    use super::*;

    pub fn add(ctx: Context<Add>) -> Result<()> {
        let v = ctx.accounts.pda.data;
        ctx.accounts.pda.data = v + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Add<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + std::mem::size_of::<Data>(),
        seeds=[],
        bump
    )]
    pda: Account<'info, Data>,
    system_program: Program<'info, System>,
}

#[account]
pub struct Data {
    data: u64,
}
