use crate::state::profile::SolanaTwitterProfile;
use anchor_lang::prelude::*;

pub fn create_profile(ctx: Context<CreateProfile>, display_name: String) -> Result<()> {
    ctx.accounts.profile.display_name = display_name;
    Ok(())
}

#[derive(Accounts)]
pub struct CreateProfile<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8+SolanaTwitterProfile::INIT_SPACE,
        seeds = [
            SolanaTwitterProfile::PROFILE_SEED.as_bytes(),
            authority.key().as_ref(),
        ],
        bump,
    )]
    pub profile: Account<'info, SolanaTwitterProfile>,
    pub system_program: Program<'info, System>,
}
