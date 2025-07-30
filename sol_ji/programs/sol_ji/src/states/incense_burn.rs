use anchor_lang::prelude::*;

use crate::states::{IncenseRule, IncenseRulesConfig, IncenseType};

#[account]
#[derive(InitSpace)]
pub struct UserBurnInfo {
    pub user: Pubkey,
    #[max_len(4)]
    pub burn_count: [u8; 4],
    // 功德值
    pub merit_value: u64,
    // 香火值
    pub incense_value: u64,
    // 烧香时间
    pub last_update_time: i64,
    // 是否重置
    pub is_reset: bool,
}

impl UserBurnInfo {
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
        self.is_reset = false;
    }
}

pub struct IncenseWallet {}
