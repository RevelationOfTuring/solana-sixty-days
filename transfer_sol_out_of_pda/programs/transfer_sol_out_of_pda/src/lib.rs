use anchor_lang::prelude::*;
use std::{mem::size_of, str::FromStr};
declare_id!("AvguBFYsWiD8F2ncemeHDN8zZs3rZuyQF37dwyHoaRKG");

const AUTHORIZER: &str = "2T1aDaVU4TFABpGUYipPQACLsL8WfAcMaUQuRK8B4itX";

#[program]
pub mod transfer_sol_out_of_pda {
    use anchor_lang::system_program;

    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn donate(ctx: Context<Donate>, amount: u64) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.signer.to_account_info().clone(),
                to: ctx.accounts.pda.to_account_info().clone(),
            },
        );

        system_program::transfer(cpi_ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        ctx.accounts.pda.sub_lamports(amount)?;
        ctx.accounts.recipent.add_lamports(amount)?;

        // in anchor 0.28 or lower, use the following syntax:
        // **ctx.accounts.pda.to_account_info().try_borrow_mut_lamports()? -= amount;
        // **ctx.accounts.signer.to_account_info().try_borrow_mut_lamports()? += amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        address = Pubkey::from_str(AUTHORIZER).unwrap()
    )]
    signer: Signer<'info>,
    #[account(
        mut,
        seeds=[],
        bump
    )]
    pda: Account<'info, Data>,
    /// CHECK: no check
    #[account(mut)]
    recipent: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct Donate<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        mut,
        seeds=[],
        bump
    )]
    pda: Account<'info, Data>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
    init,
    payer = signer,
    space = size_of::<Data>()+8,
    seeds=[],
    bump
)]
    pda: Account<'info, Data>,
    system_program: Program<'info, System>,
}

#[account]
pub struct Data {}
