use anchor_lang::prelude::*;
use anchor_spl::{metadata::{create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3, Metadata}, token::{Mint, Token}};

pub fn create_token_mint_account(ctx: Context<CreateTokenMintAccount>) -> Result<()> {
  let signer_seeds: &[&[&[u8]]] = &[&[
      b"mint_v9",
      &[ctx.bumps.token_mint_account]
    ]];
  create_metadata_accounts_v3(
    CpiContext::new_with_signer(
      ctx.accounts.token_metadata_program.to_account_info(),
      CreateMetadataAccountsV3 {
          metadata: ctx.accounts.metadata_account.to_account_info(),
          mint: ctx.accounts.token_mint_account.to_account_info(),
          mint_authority: ctx.accounts.token_mint_account.to_account_info(),
          update_authority: ctx.accounts.token_mint_account.to_account_info(),
          payer: ctx.accounts.authority.to_account_info(),
          system_program: ctx.accounts.system_program.to_account_info(),
          rent:ctx.accounts.rent.to_account_info(),
      },
      signer_seeds,
    ),
    DataV2{
      name:"Li".to_string(),
      symbol:"LI".to_string(),
      uri:"https://just-gol.github.io/".to_string(),
      seller_fee_basis_points:0,
      creators:None,
      collection:None,
      uses:None
    },
      false,
       true, 
       None
      )?;
    Ok(())
}

#[derive(Accounts)]
pub struct CreateTokenMintAccount<'info> {

  // 下面pda 是固定格式第一个必须传递 b"metadata" 第二个 token_metadata_program,第三个 mint_account
  /// CHECK: Validate address by deriving pda
  #[account(
    mut,
    seeds=[
          b"metadata",
          token_metadata_program.key().as_ref(),
          token_mint_account.key().as_ref(),
          ],
      bump,
      seeds::program = token_metadata_program.key(),
  )]
  pub metadata_account:UncheckedAccount<'info>,

    #[account(
      // 如果没有就创建
      init_if_needed, 
      payer = authority, 
      seeds=[b"mint_v9"],
      bump,
      mint::decimals = 2,
      mint::authority = token_mint_account.key(),
    )]
    pub token_mint_account: Account<'info, Mint>, // pda账户

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,

    pub token_metadata_program: Program<'info, Metadata>,

    pub rent: Sysvar<'info, Rent>,

}
