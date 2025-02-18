use anchor_lang::prelude::*;

declare_id!("4ttbU7JqGmwtwDoNVXR1FhrkypqGbuobkM7qD8Mjoyne");

#[program]
pub mod sol_spitter {

    use super::*;

    pub fn transfer_sol(ctx: Context<TransferSol>, amount: u64) -> Result<()> {
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.signer.to_account_info(),
                to: ctx.accounts.recipient.to_account_info(),
            },
        );

        let res = anchor_lang::system_program::transfer(cpi_context, amount);
        if res.is_ok() {
            return Ok(());
        } else {
            err!(Errors::FailToTransfer)
        }
    }

    // // 'a, 'b, 'c are Rust lifetimes, ignore them for now
    pub fn split_sol<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, SplitSol<'info>>,
        amount: u64,
    ) -> Result<()> {
        let amount_per_account = amount / ctx.remaining_accounts.len() as u64;
        let system_program = &ctx.accounts.system_program;
        for recipient in ctx.remaining_accounts {
            let cpi_accounts = anchor_lang::system_program::Transfer {
                from: ctx.accounts.signer.to_account_info(),
                to: recipient.to_account_info(),
            };

            let cpi_context = CpiContext::new(system_program.to_account_info(), cpi_accounts);

            let res = anchor_lang::system_program::transfer(cpi_context, amount_per_account);
            if res.is_err() {
                return err!(Errors::FailToTransfer);
            }
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SplitSol<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferSol<'info> {
    // #[account(mut)]
    pub signer: Signer<'info>,
    /// CHECK: we do not read or write the data of this account
    #[account(mut)]
    recipient: UncheckedAccount<'info>,
    system_program: Program<'info, System>,
}

#[error_code]
pub enum Errors {
    #[msg("Failed to transfer")]
    FailToTransfer,
}
