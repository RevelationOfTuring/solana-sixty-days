use anchor_lang::prelude::*;
// import bob program
use bob::program::Bob;
// import account data defination
use bob::BobData;
// account struct to build cpi context
use bob::cpi::accounts::Set;

declare_id!("GWymJaGhvWVQ5ReeF2KLD89zxscmvGV9xxxtTNUQCtKu");

#[program]
pub mod alice {
    use super::*;

    pub fn init(_ctx: Context<Init>) -> Result<()> {
        Ok(())
    }

    pub fn call_bob_to_store(ctx: Context<CallBobToStore>, data: u64) -> Result<()> {
        // call bob.set()
        bob::cpi::set(ctx.accounts.bob_set_cpi_context(), data)
    }
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        init,
        space = 8 + 8,
        payer = signer,
    )]
    alice_account: Account<'info, AliceData>,
    system_program: Program<'info, System>,
}

#[account]
pub struct AliceData {
    a: u32,
    b: u32,
}

#[derive(Accounts)]
pub struct CallBobToStore<'info> {
    #[account(mut)]
    bob_account: Account<'info, BobData>,
    bob_program: Program<'info, Bob>,
}

// add an function to CallBobToStore to get cpi context
impl<'info> CallBobToStore<'info> {
    fn bob_set_cpi_context(&self) -> CpiContext<'_, '_, '_, 'info, Set<'info>> {
        CpiContext::new(
            self.bob_program.to_account_info(),
            Set {
                bob_account: self.bob_account.to_account_info(),
            },
        )
    }
}
