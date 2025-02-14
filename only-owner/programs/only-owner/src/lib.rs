use anchor_lang::prelude::*;

declare_id!("HK933YeNBYpwKFH5QRtbF2xhumixK3ApitpBPpEAjw3c");

const OWNER: &str = "2T1aDaVU4TFABpGUYipPQACLsL8WfAcMaUQuRK8B4itX";
#[program]
pub mod only_owner {
    use super::*;

    #[access_control(check(&ctx))]
    pub fn only_owner(ctx: Context<OnlyOwner>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

fn check(ctx: &Context<OnlyOwner>) -> Result<()> {
    // check if signer == OWNER
    require_keys_eq!(
        ctx.accounts.signer.key(),
        OWNER.parse::<Pubkey>().unwrap(),
        OnlyOwnerError::NotOwner,
    );

    Ok(())
}

#[error_code]
pub enum OnlyOwnerError {
    #[msg("Only owner can call!")]
    NotOwner,
}

#[derive(Accounts)]
pub struct OnlyOwner<'info> {
    pub signer: Signer<'info>,
}
