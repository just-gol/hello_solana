use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, metadata::{create_master_edition_v3, create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMasterEditionV3, CreateMetadataAccountsV3, Metadata}, token::{mint_to, Mint, MintTo, Token, TokenAccount}};

use crate::states::BurnTokenInfoArgs;
pub fn nft_mint(ctx: Context<CreateBurnToken>, args: BurnTokenInfoArgs) -> Result<()> {
  let signer_seeds: &[&[&[u8]]] = &[&[
      b"create_burn_token",
      args.name.as_bytes(),
      &[ctx.bumps.nft_mint_account],
    ]];  
   create_metadata_accounts_v3(CpiContext::new_with_signer(
    ctx.accounts.token_metadata_program.to_account_info(), 
    CreateMetadataAccountsV3 {
        metadata: ctx.accounts.metadata_account.to_account_info(),
        mint: ctx.accounts.nft_mint_account.to_account_info(),
        mint_authority: ctx.accounts.nft_mint_account.to_account_info(),
        payer: ctx.accounts.authority.to_account_info() ,
        update_authority:ctx.accounts.nft_mint_account.to_account_info(),
        system_program:ctx.accounts.system_program.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
    },
     signer_seeds), 
     DataV2{
       name: args.name.to_string(),
        symbol: args.symbol.to_string(), 
        uri: args.url.to_string(), 
        seller_fee_basis_points: 0, 
        creators: None, collection: None, uses:None 
      }, 
      false, 
      true,
       None)?;

       mint_to(CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(),
        MintTo{
            mint: ctx.accounts.nft_mint_account.to_account_info(),
            to:ctx.accounts.nft_associated_token_account.to_account_info(),
            authority: ctx.accounts.nft_mint_account.to_account_info(),
        },
         signer_seeds),
         1)?;

         msg!("nft mint success ata:{}",ctx.accounts.nft_associated_token_account.key());

         create_master_edition_v3(CpiContext::new_with_signer(
          ctx.accounts.token_metadata_program.to_account_info(), 
          CreateMasterEditionV3{
            edition: ctx.accounts.master_editon_account.to_account_info(),
            mint: ctx.accounts.nft_mint_account.to_account_info(),
            update_authority: ctx.accounts.nft_mint_account.to_account_info(),
            mint_authority: ctx.accounts.nft_mint_account.to_account_info(),
            payer: ctx.accounts.authority.to_account_info(),
            metadata: ctx.accounts.metadata_account.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        }, 
          signer_seeds), 
          Some(1))?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(args: BurnTokenInfoArgs)]
pub struct CreateBurnToken<'info> {

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
       init,
       payer = authority, 
       seeds = [b"create_burn_token",args.name.as_bytes()],
       mint::decimals = 0,
       mint::authority = nft_mint_account,
       mint::freeze_authority = nft_mint_account,
       bump,
      )]
    pub nft_mint_account: Account<'info, Mint>,

    /// CHECK:创建唯一不可分割的nft
    #[account(
      mut,
      seeds = [b"metadata",token_metadata_program.key().as_ref(),nft_mint_account.key().as_ref(),  b"edition".as_ref(),],
      bump,
      seeds::program = token_metadata_program.key(),
    )]
    pub master_editon_account:UncheckedAccount<'info>,


    ///CHECK:
    #[account(
      mut,
      seeds = [b"metadata",token_metadata_program.key().as_ref(),nft_mint_account.key().as_ref()],
      bump,
      seeds::program = token_metadata_program.key(),
    )]
    pub metadata_account: UncheckedAccount<'info>,


    #[account(
      init_if_needed,
      payer = authority,
      associated_token::mint = nft_mint_account,
      associated_token::authority = authority,
    )]
    pub nft_associated_token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}
