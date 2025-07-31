use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)] // pda: 钱包 + 时间戳
pub struct LotteryInformationResult {
    // 抽签用户
    pub user: Pubkey,
    // 签文
    #[max_len(50)]
    pub content: String,
    // 创建时间
    pub create_at: i64,
    // 功德值
    pub merit_value: u64,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
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
pub struct LotteryArray {
    pub lottery_array: [LotteryType; 7],
}
#[account]
pub struct LotteryRecord {
    pub count: u8,
    // 当日第一次面试
    pub is_free: bool,
    // 抽签时间
    pub lottery_time: i64,
}
