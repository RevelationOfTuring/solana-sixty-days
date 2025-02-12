use anchor_lang::prelude::*;

declare_id!("7TDLiRSAvFVYsZQKYNQfZXi9ZPnToY6gbsYvW2JTAj1G");

#[program]
pub mod basic_rust {
    use super::*;

    pub fn func(_ctx: Context<Empty>, v: Vec<u64>) -> Result<()> {
        let mut new_vec = Vec::new();
        for i in v.iter() {
            if i % 2 == 0 {
                new_vec.push(*i);
            }
        }

        msg!("{:?}", new_vec);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Empty {}
