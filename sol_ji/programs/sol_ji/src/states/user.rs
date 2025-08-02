use anchor_lang::prelude::*;

use crate::states::{IncenseRulesConfig, IncenseType};

#[account]
#[derive(InitSpace)]
pub struct UserInfo {
    pub user: Pubkey,
    // 个香型烧香次数
    #[max_len(4)]
    pub burn_count: [u8; 4],
    // 功德值
    pub merit_value: u64,
    // 香火值
    pub incense_value: u64,
    // 烧香时间
    pub last_update_time: i64,

    // ==========抽签==========
    // 抽签次数
    pub lottery_count: u8,
    // 当日第一次抽签
    pub lottery_is_free: bool,
    // 抽签时间->第二天八点重置
    pub lottery_time: i64,

    // ==========许愿==========
    // 许愿次数
    pub wish_total_count: u8,
    // 许愿时间
    pub wish_update_time: i64,
    // 今日免费次数
    pub wish_daily_count: u8,
}

impl UserInfo {
    pub fn get_burn_count(&self, incense_type: IncenseType) -> u8 {
        self.burn_count[incense_type as usize]
    }

    pub fn update_user_burn_info(
        &mut self,
        user: Pubkey,
        incense_type: IncenseType,
        incense_rules_config: &IncenseRulesConfig,
    ) {
        let incense_rule = incense_rules_config.get_rule(incense_type);
        self.user = user;
        self.burn_count[incense_type as usize] += 1;
        self.merit_value += incense_rule.merit_value;
        self.incense_value += incense_rule.incense_value;
        self.last_update_time = Clock::get().unwrap().unix_timestamp;
    }

    pub fn deduction(&mut self, value: u64) {
        self.merit_value -= value;
    }

    // 抽签
    pub fn update_lottery_count(&mut self, now_ts: i64) {
        self.lottery_is_free = false;
        self.lottery_time = now_ts;
        self.lottery_count += 1;
    }

    //===许愿
    pub fn init_update_wish(&mut self) {
        self.wish_total_count = 0;
        self.wish_update_time = Clock::get().unwrap().unix_timestamp;
        self.wish_daily_count = 0;
    }

    pub fn update_user_wish_count(&mut self) {
        self.wish_total_count += 1;
        self.wish_daily_count += 1;
        self.wish_update_time = Clock::get().unwrap().unix_timestamp;
    }

    // 检查许愿
    pub fn check_is_free(&mut self) {
        let last_day = (self.wish_update_time + 8 * 3600) / 86400;
        let now_ts = Clock::get().unwrap().unix_timestamp;
        let current_day = (now_ts + 8 * 3600) / 86400;
        // 处理每日重置逻辑
        if current_day > last_day {
            self.wish_daily_count = 0;
        }
    }
}
