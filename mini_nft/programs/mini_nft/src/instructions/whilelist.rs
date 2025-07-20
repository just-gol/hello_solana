use crate::state::config::*;
use crate::state::whitelist::*;
use anchor_lang::prelude::*;
use chrono::{TimeZone, Utc};
pub fn add_whitelist(ctx: Context<AddWhitelist>, target: Pubkey) -> Result<()> {
    let start = Utc.with_ymd_and_hms(2025, 7, 15, 0, 0, 0).unwrap();
    let end = Utc.with_ymd_and_hms(2025, 7, 20, 23, 59, 59).unwrap();
    Whitelist::initialize(
        &mut ctx.accounts.whitelist,
        start.timestamp() as u64,
        end.timestamp() as u64,
        target,
    );
    Ok(())
}

#[derive(Accounts)]
pub struct AddWhitelist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    pub user: SystemAccount<'info>,

    #[account(
      init,
      payer = admin,
      space = 8+256,
      seeds=[Whitelist::PREFIX.as_bytes(), admin.key().as_ref(),user.key().as_ref()],
      bump
  )]
    pub whitelist: Account<'info, Whitelist>,

    #[account(
        seeds=[Config::PREFIX.as_bytes()],
        bump,
        has_one = admin @ ErrorCode::NotAdmin
      )]
    pub config: Account<'info, Config>,

    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Not admin")]
    NotAdmin,
}
