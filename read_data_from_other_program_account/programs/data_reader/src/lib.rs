use anchor_lang::prelude::*;

declare_id!("5yjGzs7MYLUeDyr9vVNeNWbu3Tpcpbnt52ufdmmPUuL2");

#[program]
pub mod data_reader {
    use super::*;

    pub fn read_other_account(ctx: Context<ReadOtherAccount>) -> Result<()> {
        let other_acc = &ctx.accounts.other_account;
        if other_acc.data_is_empty() {
            return err!(MyError::NoData);
        }

        // load the data from other account
        let mut data_slice: &[u8] = &other_acc.data.borrow();
        // deserialize account data into struct in memory
        let my_data: MyData = AccountDeserialize::try_deserialize(&mut data_slice)?;
        msg!("The x is {:?}", my_data.x);
        Ok(())
    }
}

#[error_code]
pub enum MyError {
    #[msg("No data")]
    NoData,
}

#[derive(Accounts)]
pub struct ReadOtherAccount<'info> {
    /// CHECK: We do not own this account so we must be very cautious with how we use the data
    other_account: UncheckedAccount<'info>,
}

// the definition of the account data
// copy from the project of data_holder
#[account]
pub struct MyData {
    x: u64,
}
