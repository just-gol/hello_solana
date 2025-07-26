use anchor_lang::prelude::*;
use anchor_spl::{metadata::{create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3, Metadata}, token::Mint};

use crate::states::{EftAsset, EtfToken};

pub fn eft_token_create(ctx: Context<EtfTokenCreate>, args: EtfTokenArgs) -> Result<()> {
  msg!("ETF Token Mint Address: {}", ctx.accounts.etf_token_mint_account.key());
  msg!("Metadata Address: {}", ctx.accounts.metadata_account.key());

  let m =  ctx.accounts.etf_token_mint_account.key();
  let signer_seeds: &[&[&[u8]]] = &[&[EtfToken::SEED_PREFIX.as_bytes(), m.as_ref(),&[ctx.bumps.etf_token_info]]];

  create_metadata_accounts_v3(CpiContext::new_with_signer(
    ctx.accounts.token_metadata_program.to_account_info(),
     CreateMetadataAccountsV3{
        metadata: ctx.accounts.metadata_account.to_account_info(),
        mint: ctx.accounts.etf_token_mint_account.to_account_info(),
        mint_authority: ctx.accounts.etf_token_info.to_account_info(),
        payer: ctx.accounts.authority.to_account_info(),
        update_authority: ctx.accounts.etf_token_info.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
     },
      signer_seeds),
       DataV2{
        name: args.name.to_string(),
        symbol:args.symbol.to_string(),
        uri: args.url.to_string(),
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    }, 
       false, 
       true, 
       None)?;

       

    ctx.accounts.etf_token_info.set_inner(
      args.create_account(ctx.accounts.authority.key(), 
      ctx.accounts.etf_token_mint_account.key())
    );

    Ok(())
}
#[account]
pub struct EtfTokenArgs {
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub url: String,
    pub assets: Vec<EftAsset>,
}

impl EtfTokenArgs {
  fn create_account(self,creator: Pubkey, mint_account: Pubkey) -> EtfToken {
    let clock =Clock::get().unwrap();
    EtfToken {
      creator,
      mint_account,
      descriptor: self.description,
      assets: self.assets,
      create_at: clock.unix_timestamp,
    }
  }
}

#[derive(Accounts)]
#[instruction(args: EtfTokenArgs)]
pub struct EtfTokenCreate<'info> {
  #[account(
    init_if_needed,
    payer = authority,
    space =8 + EtfToken::INIT_SPACE,
    seeds = [EtfToken::SEED_PREFIX.as_bytes(),etf_token_mint_account.key().as_ref()], // 一个etf_token_mint_account 对应一个 etf_token_info
    bump
  )]
  pub etf_token_info:Account<'info,EtfToken>,

  #[account(
    mut,
    seeds = [
      b"metadata", 
      token_metadata_program.key().as_ref(),
      etf_token_mint_account.key().as_ref()],
      bump,
      seeds::program = token_metadata_program.key(),
  )]
  /// CHECK
    pub metadata_account:UncheckedAccount<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
       init,
       payer = authority, 
       seeds = [EtfToken::SEED_PREFIX.as_bytes(),args.symbol.as_bytes()], // symbol 不能重读
       bump,
       mint::decimals = EtfToken::TOKEN_DECIMALS,
       mint::authority = etf_token_info.key(),
      )]
    pub etf_token_mint_account: Account<'info, Mint>,

    pub system_program : Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, anchor_spl::token::Token>,
    pub token_metadata_program: Program<'info,Metadata>,
}
