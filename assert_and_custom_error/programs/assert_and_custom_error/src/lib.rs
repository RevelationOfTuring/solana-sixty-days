use anchor_lang::prelude::*;

declare_id!("De8YAdDJKgcN19X5XoZYUDsYyMUaSpThBZoyvVNPfZhp");

#[program]
pub mod assert_and_custom_error {
    use super::*;

    pub fn assert(_ctx: Context<Assert>, a: u64) -> Result<()> {
        // if a < 10 {
        //     return err!(MyError::TooSmall);
        // } else if a > 100 {
        //     return err!(MyError::TooBig);
        // }
        require!(a >= 10, MyError::TooSmall);
        require!(a <= 100, MyError::TooBig);
        Ok(())
    }

    pub fn func(_ctx: Context<Assert>) -> Result<()> {
        msg!("Will this print?");
        return err!(MyError::AlwaysErrors);
    }
}

// custom error
#[error_code]
pub enum MyError {
    #[msg("value is too small")]
    TooSmall,
    #[msg("value is too big")]
    TooBig,
    #[msg("always errors")]
    AlwaysErrors,
}

#[derive(Accounts)]
pub struct Assert {}
