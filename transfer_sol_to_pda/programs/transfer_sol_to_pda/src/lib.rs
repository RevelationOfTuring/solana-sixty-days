use anchor_lang::prelude::*;

declare_id!("5r7jBFLyzKLqMhPPHAemrsvS13PV6E9ydaEkCpCjNSd1");

#[program]
pub mod transfer_sol_to_pda {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
