use std::mem::size_of;

use anchor_lang::prelude::*;

declare_id!("EAtEWN93PjtQcWxGGnHAF7S4EDq1y461pUvpLJzTXTPE");

#[program]
pub mod transfer_acc_ownership_in_program {
    use anchor_lang::system_program;

    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn change_owner(ctx: Context<ChangOwner>) -> Result<()> {
        let target_account_info = &mut ctx.accounts.acc.to_account_info();

        // `assign`` is the function to transfer ownership
        target_account_info.assign(&system_program::ID);


        // we must erase all the data in the account or the transfer will fail
        let res = target_account_info.realloc(0, false);
        if res.is_err() {
            return err!(Err::ReallocFailed);
        }

        Ok(())
    }
}

#[error_code]
pub enum Err {
    #[msg("realloc failed")]
    ReallocFailed,
}

#[derive(Accounts)]
pub struct ChangOwner<'info> {
    #[account(mut)]
    acc: Account<'info, MyData>,
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
    bump,
)]
    acc: Account<'info, MyData>,
    system_program: Program<'info, System>,
}

#[account]
pub struct MyData {
    x: u64,
}
