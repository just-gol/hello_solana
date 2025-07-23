use anchor_lang::prelude::*;
use anchor_spl::token::{burn, transfer, Burn, Mint, Transfer};

use crate::state::StakeInfo;
#[error_code]
pub enum UnStakeError {
    #[msg("can not unstake")]
    NoAuthority,
}
pub fn unstake(ctx: Context<NftUnstake>) -> Result<()> {
    // 校验质押关系
    require!(
        &ctx.accounts.stake_info.nft_mint_account == &ctx.accounts.nft_mint_account.key(),
        UnStakeError::NoAuthority
    );

    require!(
        &ctx.accounts.stake_info.staker == &ctx.accounts.authority.key(),
        UnStakeError::NoAuthority
    );

    // 转移nft,质押相反
    let nft_mint_account = ctx.accounts.nft_mint_account.key();

    let signer_seeds: &[&[&[u8]]] = &[&[
        b"stake_info",
        nft_mint_account.as_ref(),
        &[ctx.bumps.stake_info],
    ]];
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.program_receipt_nft_ata.to_account_info(),
                to: ctx.accounts.nft_associated_token_account.to_account_info(),
                authority: ctx.accounts.stake_info.to_account_info(),
            },
        )
        .with_signer(signer_seeds),
        1,
    )?;

    let amount = ctx.accounts.stake_info.salvage_value(10000);

    // 销毁流动性代币
    burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.token_mint_account.to_account_info(),
                from: ctx.accounts.associated_token_account.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        amount,
    )?;

    Ok(())
}
#[derive(Accounts)]
pub struct NftUnstake<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    // 接收nft的账户
    // 记录谁质押了哪个nft
    #[account(
      mut,
        seeds = [b"stake_info",nft_mint_account.key().as_ref()],
        bump,
    )]
    pub stake_info: Box<Account<'info, StakeInfo>>,

    #[account(
      mut,
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
