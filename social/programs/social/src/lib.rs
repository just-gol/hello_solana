use anchor_lang::prelude::*;

declare_id!("Ek9GNFGdLJRNVoJhzyF5WsuxUMobUSKz6CzM1o9fPzFk");
pub mod instructions;
pub mod state;
use instructions::*;

#[program]
pub mod social {
    use super::*;

    // 创建用户
    pub fn create_profile(ctx: Context<CreateProfile>, display_name: String) -> Result<()> {
        instructions::create_profile(ctx, display_name)
    }

    // 发帖
    pub fn create_tweet(ctx: Context<CreateTweet>, body: String) -> Result<()> {
        instructions::create_tweet(ctx, body)
    }

    pub fn create_like(ctx: Context<CreateLike>) -> Result<()> {
        instructions::create_like(ctx)
    }
}
