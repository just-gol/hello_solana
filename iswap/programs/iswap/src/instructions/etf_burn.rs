use std::collections::HashMap;

use crate::{
    accounts_ix::EtfTokenTransaction,
    states::{EtfToken, TokenMintError},
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::get_associated_token_address,
    token::{burn, transfer, Burn, Transfer},
};
pub fn etf_token_burn<'info>(
    ctx: Context<'_, '_, '_, 'info, EtfTokenTransaction<'info>>,
    lamports: u64,
) -> Result<()> {
    let accounts = ctx
        .remaining_accounts
        .iter()
        .map(|x| (x.key(), x.to_owned()))
        .collect::<HashMap<_, _>>();

    for x in &ctx.accounts.etf_token_info.assets {
        // 获取用户账户
        let to_ata = accounts
            .get(&get_associated_token_address(
                &ctx.accounts.authority.key(),
                &x.token,
            ))
            .ok_or(TokenMintError::InvalidAccounts)?;
        // 获取合约账户
        let from_ata = accounts
            .get(&get_associated_token_address(
                &ctx.accounts.etf_token_info.key(),
                &x.token,
            ))
            .ok_or(TokenMintError::InvalidAccounts)?;

        let amount = x.weight as u64 * lamports / 100;

        // 合约资产转移到用户
        let m = ctx.accounts.etf_token_mint_account.key();
        let signer_seeds: &[&[&[u8]]] = &[&[
            EtfToken::SEED_PREFIX.as_bytes(),
            m.as_ref(),
            &[ctx.bumps.etf_token_info],
        ]];
        transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: from_ata.to_account_info(),
                    to: to_ata.to_account_info(),
                    authority: ctx.accounts.etf_token_info.to_account_info(),
                },
                signer_seeds,
            ),
            amount,
        )?;
    }

    burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.etf_token_mint_account.to_account_info(),
                from: ctx.accounts.etf_token_ata.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        lamports,
    )?;

    Ok(())
}
