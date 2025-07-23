use crate::state::nft::Nft;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3, mpl_token_metadata::types::DataV2,
        CreateMasterEditionV3, CreateMetadataAccountsV3, Metadata,
    },
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

pub fn nft_mint(ctx: Context<NftMint>, ntf_id: String) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[b"nft", ntf_id.as_bytes(), &[ctx.bumps.nft_mint_account]]];

    create_metadata_accounts_v3(
        CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.metadata_account.to_account_info(),
                mint: ctx.accounts.nft_mint_account.to_account_info(),
                mint_authority: ctx.accounts.nft_mint_account.to_account_info(),
                payer: ctx.accounts.authority.to_account_info(),
                update_authority: ctx.accounts.nft_mint_account.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            signer_seeds,
        ),
        DataV2 {
            name: format!("{},#{}", Nft::TOKEN_NAME.to_string(), ntf_id),
            symbol: Nft::TOKEN_SYMBOL.to_string(),
            uri: Nft::TOKEN_URI.to_string(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        false,
        true,
        None,
    )?;

    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.nft_mint_account.to_account_info(),
                to: ctx.accounts.nft_associated_token_account.to_account_info(),
                authority: ctx.accounts.nft_mint_account.to_account_info(),
            },
            signer_seeds,
        ),
        1,
    )?;

    // 创建master_editon_account
    // 创建唯一且不可分割 NFT 的标准流程之一
    create_master_edition_v3(
        CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMasterEditionV3 {
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
            signer_seeds,
        ),
        Some(1),
    )?;
    Ok(())
}

#[derive(Accounts)]
#[instruction(ntf_id: String)]
pub struct NftMint<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    // 创建meta相关账户
    // seeds = ["metadata", metadata_program_id, mint_address, "edition"]
    #[account(
    mut,
    seeds=[
      b"metadata",
      token_metadata_program.key().as_ref(),
      nft_mint_account.key().as_ref(),
      b"edition".as_ref(),
    ],
    bump,
    seeds::program = token_metadata_program.key(),
  )]
    /// CHECK
    pub master_editon_account: UncheckedAccount<'info>,

    // seeds = ["metadata", metadata_program_id, mint_address]
    #[account(
    mut,
    seeds=[
      b"metadata",
      token_metadata_program.key().as_ref(),
      nft_mint_account.key().as_ref(),
    ],
    bump,
    seeds::program = token_metadata_program.key(),
  )]
    /// CHECK
    pub metadata_account: UncheckedAccount<'info>,

    // 创建mint 账户
    #[account(
    init,
    payer = authority,
    mint::decimals=0,
    mint::authority = nft_mint_account,
    mint::freeze_authority = nft_mint_account,
    seeds = [Nft::SEED_PREFIX, &ntf_id.to_string().as_bytes()],
    bump,
  )]
    pub nft_mint_account: Account<'info, Mint>,

    // 创建ata
    #[account(
      init_if_needed,
      payer = authority,
      associated_token::mint = nft_mint_account,
      associated_token::authority = authority,
    )]
    pub nft_associated_token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub rent: Sysvar<'info, Rent>,
}
