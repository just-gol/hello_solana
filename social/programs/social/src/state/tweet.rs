use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct SolanaTwitterTweet {
    // 点赞数
    pub like_count: u64,

    // 作者
    pub author: Pubkey,

    #[max_len(50)]
    pub body: String,
}

impl SolanaTwitterTweet {
    pub const TWEET_SEED: &'static str = "tweet";
    pub fn new(body: String, author: Pubkey) -> Self {
        Self {
            body,
            like_count: 0,
            author,
        }
    }
}
