use crate::state::counter::*;
use anchor_lang::prelude::*;
pub fn increment(ctx: Context<AccountCounter>, author: Pubkey) -> Result<()> {
    require!(
        author == ctx.accounts.authority.key(),
        ErrorCode::InvalidAccount
    );
    Counter::increment(&mut ctx.accounts.counter);
    Ok(())
}

pub fn decriment(ctx: Context<AccountCounter>, author: Pubkey) -> Result<()> {
    require!(
        author == ctx.accounts.authority.key(),
        ErrorCode::InvalidAccount
    );
    Counter::decriment(&mut ctx.accounts.counter);
    Ok(())
}

#[derive(Accounts)]
pub struct AccountCounter<'info> {
    #[account(
      init_if_needed,
      payer = authority,
      space = 8 +256,
      seeds = [Counter::PREFIX.as_bytes(), authority.key().as_ref()],
      bump
    )]
    pub counter: Account<'info, Counter>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    InvalidAccount,
}
