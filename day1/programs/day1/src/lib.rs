use anchor_lang::prelude::*;

declare_id!("D2oQMr63H32W4QwT6K5KpGZHF2VLXMZe43cnqGYXXqiH");

#[program]
pub mod day1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello world {:?}", ctx.accounts.signer.key);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub signer: Signer<'info>,
}
