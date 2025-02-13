use anchor_lang::prelude::*;

declare_id!("E54ShTpgyJzJ3hFWPCpLVugLcWHogYeqehVVU1VwERWv");

#[program]
pub mod sysvar {
    use anchor_lang::solana_program::{
        last_restart_slot::LastRestartSlot, sysvar::recent_blockhashes::RecentBlockhashes,
    };
    use chrono::Datelike;

    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        let clock = Clock::get()?;
        msg!("Block timestamp: {}", clock.unix_timestamp);
        Ok(())
    }

    pub fn get_day_of_the_week(_ctx: Context<Initialize>) -> Result<()> {
        let clock = Clock::get()?;
        let date_time = chrono::NaiveDateTime::from_timestamp_opt(clock.unix_timestamp, 0).unwrap();
        let the_day_of_week = date_time.weekday();
        msg!("the day of week: {}", the_day_of_week);
        Ok(())
    }

    pub fn get_latest_blockhashes(ctx: Context<GetRecentBlockhashes>) -> Result<()> {
        let arr = [ctx.accounts.recent_blockhashes.clone()];
        let account_iter = &mut arr.iter();
        let sh_sysvar_info = next_account_info(account_iter)?;
        let recent_blockhashes: RecentBlockhashes =
            RecentBlockhashes::from_account_info(sh_sysvar_info)?;
        let lastest_blockhash = recent_blockhashes.last().unwrap();
        msg!("{:?}", lastest_blockhash.blockhash);
        msg!("{:?}", lastest_blockhash.fee_calculator);
        Ok(())
    }

    pub fn get_epoch_schedule(_ctx: Context<Initialize>) -> Result<()> {
        let epoch_schedule = EpochSchedule::get()?;
        msg!("epoch schedule: {:?}", epoch_schedule);
        Ok(())
    }

    pub fn get_rent(_ctx: Context<Initialize>) -> Result<()> {
        let rent = Rent::get()?;
        msg!("rent: {:?}", rent);
        Ok(())
    }

    pub fn get_instruction(ctx: Context<GetInstruction>, number: u32) -> Result<()> {
        // Get Instruction sysvar
        let arr = [ctx.accounts.instruction_sysvar.clone()];
        let account_info_iter = &mut arr.iter();
        let instructions_sysvar_account = next_account_info(account_info_iter)?;
        // Load the instruction details from the instruction sysvar account
        let instruction_details =
            anchor_lang::solana_program::sysvar::instructions::load_instruction_at_checked(
                0,
                instructions_sysvar_account,
            )?;

        msg!(
            "Instruction details of this transaction: {:?}",
            instruction_details
        );
        msg!("Number is: {}", number);

        Ok(())
    }

    pub fn get_last_restart_slot(ctx: Context<GetLastRestartSlot>) -> Result<()> {
        // Create an array to store the LastRestartSlot account
        let arr = [ctx.accounts.last_restart_slot.clone()];
        // Create an iterator for the array
        let accounts_iter = &mut arr.iter();
        // Get the next account info from the iterator (still LastRestartSlot)
        let lrs_sysvar_info = next_account_info(accounts_iter)?;
        // Create a LastRestartSlot instance from the account info
        let last_restart_slot = LastRestartSlot::from_account_info(lrs_sysvar_info)?;
        msg!("last_restart_slot: {:?}", last_restart_slot);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct GetLastRestartSlot<'info> {
    /// CHECK: readonly
    pub last_restart_slot: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct GetInstruction<'info> {
    /// CHECK:
    pub stake_history: AccountInfo<'info>,
    /// CHECK:
    pub recent_blockhashes: AccountInfo<'info>,
    /// CHECK:
    pub instruction_sysvar: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct GetRecentBlockhashes<'info> {
    /// CHECK: readonly
    pub recent_blockhashes: AccountInfo<'info>,
}
