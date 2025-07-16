use anchor_lang::prelude::*;

use crate::state::{SolanaTwitterLike, SolanaTwitterProfile, SolanaTwitterTweet};
pub fn create_tweet(ctx: Context<CreateTweet>, body: String) -> Result<()> {
    msg!("INIT_SPACE: {}", SolanaTwitterTweet::INIT_SPACE);
    msg!("body bytes len: {}", body.as_bytes().len());
    let profile = &mut ctx.accounts.profile;
    profile.tweet_count += 1;
    let tweet = SolanaTwitterTweet::new(body);
    ctx.accounts.tweet.set_inner(tweet.clone());
    Ok(())
}

#[derive(Accounts)]
pub struct CreateTweet<'info> {
    // pda 根据用户钱包 + tweet + tweet_count生成
    #[account(
        init,
        payer = authority,
        space = 8 + SolanaTwitterTweet::INIT_SPACE,
        seeds = [
            SolanaTwitterTweet::TWEET_SEED.as_bytes(),
            profile.key().as_ref(),
            (profile.tweet_count + 1).to_string().as_bytes(),
        ],
        bump,
    )]
    pub tweet: Account<'info, SolanaTwitterTweet>,

    // 用户 pda 账户
    #[account(
        mut,
        seeds = [
            SolanaTwitterProfile::PROFILE_SEED.as_bytes(),
            authority.key().as_ref(),
        ],
        bump,
    )]
    pub profile: Account<'info, SolanaTwitterProfile>,

    // 用户钱包，用于签名和支付费用
    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
pub fn create_like(ctx: Context<CreateLike>) -> Result<()> {
    let tweet = &mut ctx.accounts.tweet;
    tweet.like_count += 1;

    let like_rel = SolanaTwitterLike::new(ctx.accounts.profile.key(), tweet.key());
    ctx.accounts.like.set_inner(like_rel);
    Ok(())
}

#[derive(Accounts)]
pub struct CreateLike<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    // 点赞前缀 + 点赞人 + 推文
    #[account(
        init,
        payer = authority,
        space = 8+SolanaTwitterLike::INIT_SPACE,
        seeds=[
              SolanaTwitterLike::SEED_LIKE.as_bytes(),
              profile.key().as_ref(),
              tweet.key().as_ref()
            ],
        bump
    )]
    pub like: Account<'info, SolanaTwitterLike>,

    #[account(mut)]
    pub tweet: Account<'info, SolanaTwitterTweet>,

    // 点赞的tweet
    #[account(
        mut,
        seeds = [
            SolanaTwitterProfile::PROFILE_SEED.as_bytes(),
            authority.key().as_ref(),
        ],
        bump,

    )]
    // 点赞的人
    pub profile: Account<'info, SolanaTwitterProfile>,

    // 使用init 需要添加系统合约
    pub system_program: Program<'info, System>,
}
