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

    // 创建token mint账户
    pub fn create_token_mint_account(ctx: Context<CreateTokenMintAccount>) -> Result<()> {
        instructions::create_token_mint_account(ctx)
    }

    // 点赞
    pub fn create_like(ctx: Context<CreateLike>) -> Result<()> {
        instructions::create_like(ctx)
    }

    // nft mint
    pub fn nft_mint(ctx: Context<NftMint>, ntf_id: String) -> Result<()> {
        instructions::mpl_token_metadata::nft_mint(ctx, ntf_id)
    }

    // 质押
    pub fn stake(ctx: Context<NftStake>) -> Result<()> {
        instructions::nft_stake::stake(ctx)
    }

    // 解质押
    pub fn unstake(ctx: Context<NftUnstake>) -> Result<()> {
        instructions::nft_unstake::unstake(ctx)
    }
}
