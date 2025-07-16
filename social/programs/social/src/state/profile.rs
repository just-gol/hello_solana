use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct SolanaTwitterProfile {
    // 该用户发布的tweet数量
    pub tweet_count: u32,

    // 昵称
    #[max_len(100)]
    pub display_name: String,
}

impl SolanaTwitterProfile {
    pub const PROFILE_SEED: &'static str = "profile";
}
