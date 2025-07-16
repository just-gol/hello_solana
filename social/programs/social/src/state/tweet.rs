use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct SolanaTwitterTweet {
    // 点赞数
    pub like_count: u64,
    #[max_len(50)]
    pub body: String,
}

impl SolanaTwitterTweet {
    pub const TWEET_SEED: &'static str = "tweet";
    pub fn new(body: String) -> Self {
        Self {
            body,
            like_count: 0,
        }
    }
}
