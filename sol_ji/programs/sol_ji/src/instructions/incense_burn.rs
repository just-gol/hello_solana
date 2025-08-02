use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, system_instruction::transfer},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{burn, Burn, Mint, Token, TokenAccount, Transfer},
};

use crate::states::{IncenseRulesConfig, IncenseType, UserInfo};
pub fn incense_burn(ctx: Context<CreateIncense>, a: IncenseType) -> Result<()> {
    let user_burn_info = &mut ctx.accounts.user_burn_info;
    msg!("user_burn_info:{}", user_burn_info.key());
    // 当前 UTC 时间 -> DateTime
    let now_ts = Clock::get()?.unix_timestamp;

    check_daily_reset_and_limit(user_burn_info, now_ts, a)?;

    // 获取香的规则
    let incense_rules_config = &mut ctx.accounts.incense_rules_config;

    // 转账
    let amount = incense_rules_config.get_rule(a).incense_price;
    let tx = transfer(
        &ctx.accounts.authority.key(),
        &ctx.accounts.treasury.key(),
        amount,
    );

    invoke(
        &tx,
        &[
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.treasury.to_account_info(),
        ],
    )?;

    msg!("Transfer success");

    // 转移nft
    anchor_spl::token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.nft_associated_token_account.to_account_info(),
                to: ctx.accounts.user_receive_nft_ata.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        1,
    )?;

    msg!(
        "transfer nft success,to ata:{}",
        ctx.accounts.user_receive_nft_ata.key()
    );

    user_burn_info.update_user_burn_info(ctx.accounts.authority.key(), a, incense_rules_config);
    Ok(())
}

pub fn destroy(ctx: Context<Destroy>) -> Result<()> {
    let m = ctx.accounts.authority.key();
    let signer_seeds: &[&[&[u8]]] =
        &[&[b"user_burn_info", m.as_ref(), &[ctx.bumps.user_burn_info]]];
    burn(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.nft_mint_account.to_account_info(),
                from: ctx.accounts.user_receive_nft_ata.to_account_info(),
                authority: ctx.accounts.user_burn_info.to_account_info(),
            },
            signer_seeds,
        ),
        1,
    )?;
    Ok(())
}

// 提取的检查函数
pub fn check_daily_reset_and_limit(
    user_burn_info: &mut Account<UserInfo>,
    now_ts: i64,
    incense_type: IncenseType,
) -> Result<()> {
    let last_day = (user_burn_info.last_update_time + 8 * 3600) / 86400;
    let current_day = (now_ts + 8 * 3600) / 86400;

    // 处理每日重置逻辑
    if current_day > last_day {
        if user_burn_info.burn_count.iter().any(|&x| x != 0) {
            user_burn_info.burn_count = [0; 4];
            user_burn_info.last_update_time = now_ts;
        }
    }

    // 检查是否超过最大次数
    if user_burn_info.get_burn_count(incense_type) >= 10 {
        return Err(BurnCode::TooManyBurns.into());
    }

    Ok(())
}

#[derive(Accounts)]
pub struct CreateIncense<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
      mut,
      seeds = [b"incense_rules_config"] ,
      bump
  )]
    pub incense_rules_config: Account<'info, IncenseRulesConfig>,

    /// CHECK:
    #[account(
      init_if_needed,
      payer = authority,
      space = 8,
      seeds = [b"treasury_incense_burn"],
      bump
    )]
    pub treasury: UncheckedAccount<'info>, // 接收SOL

    #[account(
        mut,
        associated_token::mint = nft_mint_account,
        associated_token::authority = authority,
      )]
    pub nft_associated_token_account: Account<'info, TokenAccount>, // 转移NFT的账户

    #[account(mut)]
    pub nft_mint_account: Account<'info, Mint>,

    #[account(
      init_if_needed,
      payer = authority,
      space = 8 + UserInfo::INIT_SPACE,
      seeds = [b"user_burn_info",authority.key().as_ref()],
      bump
    )]
    pub user_burn_info: Account<'info, UserInfo>,

    // 接收nft账户
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = nft_mint_account,
        associated_token::authority = user_burn_info,
      )]
    pub user_receive_nft_ata: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program: Program<'info, Token>,

    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Destroy<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub nft_mint_account: Account<'info, Mint>,

    #[account(
      mut,
      seeds = [b"user_burn_info",authority.key().as_ref()],
      bump
    )]
    pub user_burn_info: Account<'info, UserInfo>,

    // 接收nft账户
    #[account(
        mut,
        associated_token::mint = nft_mint_account,
        associated_token::authority = user_burn_info,
      )]
    pub user_receive_nft_ata: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program: Program<'info, Token>,

    pub rent: Sysvar<'info, Rent>,
}

#[error_code]
pub enum BurnCode {
    #[msg("Too many burns")]
    TooManyBurns,
}
