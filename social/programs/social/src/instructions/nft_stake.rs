use anchor_lang::{prelude::*, solana_program::stake::state::Stake};
use anchor_spl::token::{mint_to, transfer, Mint, MintTo, Transfer};

use crate::state::StakeInfo;

pub fn stake(ctx: Context<NftStake>) -> Result<()> {
    // 记录质押关系
    let stake_info = StakeInfo::new(
        ctx.accounts.authority.key(),
        ctx.accounts.nft_mint_account.key(),
    );

    ctx.accounts.stake_info.set_inner(stake_info.clone());

    // 转移nft
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.nft_associated_token_account.to_account_info(),
                to: ctx.accounts.program_receipt_nft_ata.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        1,
    )?;

    // mint 流动性代币
    let signer_seeds: &[&[&[u8]]] = &[&[b"mint_v9", &[ctx.bumps.token_mint_account]]];
    mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.token_mint_account.to_account_info(),
                to: ctx.accounts.associated_token_account.to_account_info(),
                authority: ctx.accounts.token_mint_account.to_account_info(),
            },
        )
        .with_signer(signer_seeds),
        10000,
    )?;
    Ok(())
}

#[derive(Accounts)]
pub struct NftStake<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    // 接收nft的账户
    // 记录谁质押了哪个nft
    #[account(
      init_if_needed,
        payer = authority,
        space = 8 + StakeInfo::INIT_SPACE, // 质押信息的空间
        seeds = [b"stake_info",nft_mint_account.key().as_ref()],
        bump
    )]
    pub stake_info: Box<Account<'info, StakeInfo>>,

    #[account(
      init_if_needed,
      payer = authority,
      associated_token::mint = nft_mint_account,
      associated_token::authority = stake_info,
    )]
    pub program_receipt_nft_ata: Box<Account<'info, anchor_spl::token::TokenAccount>>,

    // 流动性token
    #[account(
      mut,
      seeds=[b"mint_v9"],
      bump,
    )]
    pub token_mint_account: Box<Account<'info, Mint>>,
    // 关联的token账户
    #[account(
      init_if_needed,
      payer = authority,
      associated_token::mint = token_mint_account,
      associated_token::authority = authority,
    )]
    pub associated_token_account: Box<Account<'info, anchor_spl::token::TokenAccount>>,

    // 质押的nft
    #[account(mut)]
    pub nft_mint_account: Box<Account<'info, Mint>>,

    #[account(
      mut,
      associated_token::mint = nft_mint_account,
      associated_token::authority = authority,
    )]
    pub nft_associated_token_account: Box<Account<'info, anchor_spl::token::TokenAccount>>,

    pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, anchor_spl::token::Token>,
}
