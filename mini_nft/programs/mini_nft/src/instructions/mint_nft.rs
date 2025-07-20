use crate::state::whitelist::*;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

pub fn mint_nft(ctx: Context<MintNft>) -> Result<()> {
  msg!("is_minted: {}, target: {}", ctx.accounts.whitelist.is_minted, ctx.accounts.whitelist.target.to_string());

    // 判断时间是否合法
    if ctx.accounts.whitelist.mint_start_time > Clock::get()?.unix_timestamp as u64
        || ctx.accounts.whitelist.mint_end_time < Clock::get()?.unix_timestamp as u64
    {
      return Err(error!(ErrorCode::TimeError));
    }

    if ctx.accounts.whitelist.target != ctx.accounts.user.key() {
      return Err(error!(ErrorCode::NotWhitelist));
    }

    let signer_seeds: &[&[&[u8]]] = &[&[b"mint_nft", &[ctx.bumps.nft_mint]]];
    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.nft_mint.to_account_info(),
                to: ctx.accounts.user_ata.to_account_info(),
                authority: ctx.accounts.nft_mint.to_account_info(),
            },
            signer_seeds,
        ),
        1000000,
    )?;

    ctx.accounts.whitelist.is_minted = true;
    Ok(())
}

#[derive(Accounts)]
pub struct MintNft<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
      mut,  
      seeds=[
        Whitelist::PREFIX.as_bytes(), 
        admin.key().as_ref(),user.key().as_ref()],
        bump,
        constraint = !whitelist.is_minted @ ErrorCode::AlreadyMinted
  )]
    pub whitelist: Account<'info, Whitelist>,

    pub admin: SystemAccount<'info>,

    #[account(
      init_if_needed,
      payer = user,
      seeds=[b"mint_nft"],
      mint::decimals = 2,
      mint::authority = nft_mint.key(),
      bump
    )]
    pub nft_mint: Account<'info, Mint>,

    #[account(
      init_if_needed ,
      payer = user,
      associated_token::mint = nft_mint,
      associated_token::authority = user,
    )]
    pub user_ata: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub rent: Sysvar<'info, Rent>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Already minted")]
    AlreadyMinted,

    #[msg("NotInWhitelist")]
    NotInWhitelist,
    #[msg("TimeError")]
    TimeError,
    #[msg("NotWhitelist")]
    NotWhitelist
}
