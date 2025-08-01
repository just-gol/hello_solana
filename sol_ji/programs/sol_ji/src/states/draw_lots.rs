use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)] // pda: 钱包 + 时间戳
pub struct LotteryRecord {
    // 抽签用户
    pub user: Pubkey,
    // 签文
    #[max_len(50)]
    pub lottery_type: LotteryType,
    // 创建时间
    pub create_at: i64,
    // 功德值
    pub merit_value: u64,
}

impl LotteryRecord {
    pub fn new(user: Pubkey, lottery_type: LotteryType, create_at: i64, merit_value: u64) -> Self {
        Self {
            user,
            lottery_type,
            create_at,
            merit_value,
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum LotteryType {
    // 大吉
    GreatFortune,
    //  中吉
    MiddleFortune,
    // 小吉
    SmallFortune,
    // 吉
    Fortune,
    // 末吉
    LateFortune,
    // 凶
    Misfortune,
    // 大凶
    GreatMisfortune,
}
// 初始化签文
#[account]
#[derive(InitSpace)]
pub struct LotteryConfig {
    #[max_len(7)]
    pub lottery_array: [LotteryType; 7],
}

impl LotteryConfig {
    pub fn new() -> Self {
        Self {
            lottery_array: [
                LotteryType::GreatFortune,
                LotteryType::MiddleFortune,
                LotteryType::SmallFortune,
                LotteryType::Fortune,
                LotteryType::LateFortune,
                LotteryType::Misfortune,
                LotteryType::GreatMisfortune,
            ],
        }
    }

    pub fn get_lottery_type(&self, index: u64) -> LotteryType {
        self.lottery_array[index as usize]
    }
}

#[account]
#[derive(InitSpace)]
pub struct LotteryCount {
    pub count: u8,
    // 当日第一次抽签
    pub is_free: bool,
    // 抽签时间->第二天八点重置
    pub lottery_time: i64,
}

impl LotteryCount {
    pub fn update_lottery_count(&mut self, now_ts: i64) {
        self.is_free = false;
        self.lottery_time = now_ts;
        self.count += 1;
    }
}
