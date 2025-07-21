use anchor_lang::prelude::*;

declare_id!("8puG2zVX78ctecuTBGQLkZAVDBjnBrsMuyESZDnDtDCa");
mod instructions;
mod state;
use instructions::*;

#[program]
pub mod content_platform {
    use super::*;

    // 用户发布文章
    pub fn create_article(
        ctx: Context<CreateArticle>,
        title: String,
        content: String,
    ) -> Result<()> {
        instructions::create_article(ctx, title, content)
    }

    pub fn view_articles(ctx: Context<ViewArticles>, offset: u64, limit: u64) -> Result<()> {
        instructions::view_articles(ctx, offset, limit)
    }

    pub fn create_mint(ctx: Context<CreateMint>) -> Result<()> {
        instructions::create_mint(ctx)
    }

    pub fn reward_author(ctx: Context<RewardAuthor>) -> Result<()> {
        instructions::reward_author(ctx)
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        instructions::withdraw(ctx)
    }
}
