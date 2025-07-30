use anchor_lang::prelude::*;

use crate::states::{IncenseRulesConfig, IncenseType, UserBurnInfo};
pub fn burn(ctx: Context<Burn>, a: IncenseType) -> Result<()> {
    let user_burn_info = &mut ctx.accounts.user_burn_info;
    // 当前 UTC 时间 -> DateTime
    let now_ts = Clock::get()?.unix_timestamp;

    check_daily_reset_and_limit(user_burn_info, now_ts, a)?;

    // 转账

    // 获取香的规则
    let incense_rules_config = &mut ctx.accounts.incense_rules_config;
    user_burn_info.update_user_burn_info(ctx.accounts.authority.key(), a, incense_rules_config);
    Ok(())
}

// 提取的检查函数
pub fn check_daily_reset_and_limit(
    user_burn_info: &mut Account<UserBurnInfo>,
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
pub struct Burn<'info> {
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
    pub treasury: UncheckedAccount<'info>,

    #[account(
      init_if_needed,
      payer = authority,
      space = 8 + UserBurnInfo::INIT_SPACE,
      seeds = [b"user_burn_info", authority.key().as_ref()],
      bump
    )]
    pub user_burn_info: Account<'info, UserBurnInfo>,

    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum BurnCode {
    #[msg("Too many burns")]
    TooManyBurns,
}
