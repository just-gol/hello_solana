use std::collections::HashMap;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::get_associated_token_address,
    token::{mint_to, transfer, Mint, MintTo, Transfer},
};

use crate::states::{EtfToken, TokenMintError};

/**
 * lamports 购买etf数量
 */
pub fn etf_token_mint<'info>(
    ctx: Context<'_, '_, '_, 'info, EtfTokenTransaction<'info>>,
    lamports: u64,
) -> Result<()> {
    // 存储不定长的账户 ,账户的address 作为key 账户本身 作为value
    let accounts = ctx
        .remaining_accounts
        .iter()
        .map(|x| (x.key(), x.to_owned()))
        .collect::<HashMap<_, _>>();
    // 获取资产
    for x in &ctx.accounts.etf_token_info.assets {
        // 用户的ata账户
        let from_ata = accounts
            .get(&get_associated_token_address(
                &ctx.accounts.authority.key(),
                &x.token,
            ))
            .ok_or(TokenMintError::InvalidAccounts)?;

        let to_ata = accounts
            .get(&get_associated_token_address(
                &ctx.accounts.etf_token_info.key(),
                &x.token,
            ))
            .ok_or(TokenMintError::InvalidAccounts)?;

        let amount = x.weight as u64 * lamports / 100;

        // 用户资产转移
        transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: from_ata.to_account_info(),
                    to: to_ata.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
            ),
            amount,
        )?;
    }
    // 给用户mint 相应的token
    let m = ctx.accounts.etf_token_mint_account.key();
    let signer_seeds: &[&[&[u8]]] = &[&[
        EtfToken::SEED_PREFIX.as_bytes(),
        m.as_ref(),
        &[ctx.bumps.etf_token_info],
    ]];
    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.etf_token_mint_account.to_account_info(),
                to: ctx.accounts.etf_token_ata.to_account_info(),
                authority: ctx.accounts.etf_token_info.to_account_info(),
            },
            signer_seeds,
        ),
        lamports,
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct EtfTokenTransaction<'info> {
    // etf信息和 token资产
    #[account(
        mut,
        seeds = [EtfToken::SEED_PREFIX.as_bytes(), etf_token_mint_account.key().as_ref()],
        bump,)]
    pub etf_token_info: Account<'info, EtfToken>,

    #[account(mut)]
    pub etf_token_mint_account: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = etf_token_mint_account,
        associated_token::authority = authority,
    )]
    pub etf_token_ata: Account<'info, anchor_spl::token::TokenAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, anchor_spl::token::Token>,

    pub system_program: Program<'info, System>,

    pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
}
