use anchor_lang::prelude::*;

declare_id!("4uaije4V27Uqbu7dnfP31TeD7ZmdB1Jh7w6T7eGtPVQA");

#[program]
pub mod day2 {
    use super::*;

    pub fn array(_ctx: Context<Initialize>, arr: Vec<u64>, a: u64, s: String) -> Result<()> {
        msg!("array [{:?}], u64 [{}], String [{}]", arr, a, s);
        Ok(())
    }

    pub fn f1(_ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let c = a + b;
        msg!("{}", c);
        Ok(())
    }
    pub fn f2(_ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let c = a.checked_add(b).unwrap();
        msg!("{}", c);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
