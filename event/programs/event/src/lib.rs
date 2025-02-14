use anchor_lang::prelude::*;

declare_id!("HLaWnVCtApiEGS2Tx4Z7p1PE1YoyDaC7kYAeV4EGBFu2");

#[program]
pub mod event {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>, s: String) -> Result<()> {
        let l = s.len();
        emit!(MyEvent1 { i: l as u64 });
        emit!(MyEvent2 {
            i: l as i32,
            msg: s
        });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[event]
pub struct MyEvent1 {
    i: u64,
}

#[event]
pub struct MyEvent2 {
    i: i32,
    msg: String,
}
