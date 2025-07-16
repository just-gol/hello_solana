use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct SolanaTwitterLike {
    // 点赞的人
    pub profile_pubkey: Pubkey,
    // 点赞的推文
    pub tweet_pubkey: Pubkey,
}

impl SolanaTwitterLike {
    pub const SEED_LIKE: &'static str = "like";

    pub fn new(profile_pubkey: Pubkey, tweet_pubkey: Pubkey) -> Self {
        Self {
            profile_pubkey,
            tweet_pubkey,
        }
    }
}
