use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::states::{LotteryConfig, LotteryRecord, UserInfo};
pub fn initialize_lottery_poetry(ctx: Context<InitializeLotteryPoetry>) -> Result<()> {
    let config = LotteryConfig::new();
    ctx.accounts.lottery_array.set_inner(config);
    msg!("Initialization successful");
    Ok(())
}

// value是扣除功德值
pub fn draw_lots(ctx: Context<DrawLots>, value: u64) -> Result<()> {
    let now_ts = Clock::get()?.unix_timestamp;
    msg!("当前链上时间戳: {}", now_ts);

    // 获取下表随机数
    let random = (ctx.accounts.authority.key().as_ref()[0] as u64 + now_ts as u64) % 7;
    let lottery_type = ctx.accounts.lottery_array.get_lottery_type(random);
    msg!("随机数: {},抽签结果:{:?}", random, lottery_type);

    // 判断是否第一次抽签
    let user_burn_info = &mut ctx.accounts.user_burn_info;

    check_is_free(user_burn_info, now_ts);

    // 扣除功德值
    if !user_burn_info.lottery_is_free {
        msg!("扣减钱功能的值: {}", user_burn_info.merit_value);
        if user_burn_info.merit_value < value {
            return Err(DrawLotsCode::Insufficient.into());
        }
        user_burn_info.merit_value -= value;
        msg!("剩余功德值: {}", user_burn_info.merit_value)
    }

    // 创建抽签记录
    let merit_value = if user_burn_info.lottery_is_free {
        0
    } else {
        value
    };
    let lottery_record = LotteryRecord::new(
        ctx.accounts.authority.key(),
        lottery_type,
        now_ts,
        merit_value,
    );

    ctx.accounts.lottery_record.set_inner(lottery_record);

    // 更新抽签次数
    user_burn_info.update_lottery_count(now_ts);

    Ok(())
}

pub fn check_is_free(user_burn_info: &mut UserInfo, now_ts: i64) {
    if user_burn_info.lottery_time == 0 {
        user_burn_info.lottery_is_free = true;
        return;
    }

    let last_day = (user_burn_info.lottery_time + 8 * 3600) / 86400;
    let current_day = (now_ts + 8 * 3600) / 86400;

    if current_day > last_day {
        user_burn_info.lottery_is_free = true; // 每天第一次默认免费
    }
}

#[derive(Accounts)]
pub struct InitializeLotteryPoetry<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + LotteryConfig::INIT_SPACE,
        seeds = [b"lottery_array"],
        bump
    )]
    pub lottery_array: Account<'info, LotteryConfig>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DrawLots<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
      mut,
      seeds = [b"lottery_array"],
      bump
    )]
    pub lottery_array: Account<'info, LotteryConfig>,

    // 存储每次抽签结果
    #[account(
      init,
      payer = authority,
      space = 8 + LotteryRecord::INIT_SPACE,
      seeds = [b"lottery_record",authority.key().as_ref(),(user_burn_info.lottery_count+1).to_string().as_bytes()], 
      bump
    )]
    pub lottery_record: Account<'info, LotteryRecord>,

    // 功德值->在这个账户中
    #[account(
      mut,
      seeds = [b"user_burn_info",authority.key().as_ref()],
      bump
    )]
    pub user_burn_info: Account<'info, UserInfo>,

    #[account(mut)]
    pub nft_mint_account: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum DrawLotsCode {
    #[msg("insufficient merit value")]
    Insufficient,
}
