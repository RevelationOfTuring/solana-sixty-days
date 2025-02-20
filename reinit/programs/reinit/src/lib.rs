use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("GYyDTGVktVAPWXhkMGzxQVTQVcYQQsPs6A2d4wQA8p3o");

#[program]
pub mod reinit {
    use anchor_lang::system_program;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.pda.x = 1024;
        Ok(())
    }

    pub fn drain_all_balance_from_pda(ctx: Context<DrainAllBalanceFromPda>) -> Result<()> {
        let pda_account_info = &mut ctx.accounts.pda.to_account_info();
        let all_balance = pda_account_info.lamports();
        pda_account_info.sub_lamports(all_balance)?;
        ctx.accounts
            .signer
            .to_account_info()
            .add_lamports(all_balance)?;
        Ok(())
    }

    pub fn transfer_ownership_to_system_program(
        ctx: Context<TransferOwnershipToSystemProgram>,
    ) -> Result<()> {
        let pda_account_info = &mut ctx.accounts.pda.to_account_info();
        pda_account_info.assign(&system_program::ID);
        pda_account_info.realloc(0, false)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferOwnershipToSystemProgram<'info> {
    #[account(
        mut,
        seeds = [],
        bump
    )]
    pda: Account<'info, Pda>,
}

#[derive(Accounts)]
pub struct DrainAllBalanceFromPda<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        mut,
        seeds = [],
        bump
    )]
    pda: Account<'info, Pda>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        init,
        space = 8 + size_of::<Pda>(),
        payer = signer,
        seeds = [],
        bump
    )]
    pda: Account<'info, Pda>,
    system_program: Program<'info, System>,
}

#[account]
pub struct Pda {
    x: u64,
}
