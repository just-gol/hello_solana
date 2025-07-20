use crate::state::config::*;
use anchor_lang::prelude::*;
pub fn set_admin(ctx: Context<SetAdmin>, admin: Pubkey) -> Result<()> {
    Config::initialize(&mut ctx.accounts.config, admin);
    Ok(())
}

#[derive(Accounts)]
pub struct SetAdmin<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
      init,
      payer = admin,
      space = 8+32,
      seeds=[Config::PREFIX.as_bytes()],
      bump,
    )]
    pub config: Account<'info, Config>,

    pub system_program: Program<'info, System>,
}
