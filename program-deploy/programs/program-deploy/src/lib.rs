use anchor_lang::prelude::*;

declare_id!("Eq3bfo6eCuv5xhKvmFgWRCvkfTMfG5UxXEwm9Cwa23jE");

#[program]
pub mod program_deploy {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        msg!("Hello after upgradition: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
