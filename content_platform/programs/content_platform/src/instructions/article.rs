use crate::state::article::Article;
use crate::state::author::AuthorProfile;
use anchor_lang::prelude::*;
pub fn create_article(ctx: Context<CreateArticle>, title: String, content: String) -> Result<()> {
    let article = &mut ctx.accounts.article;
    article.author = ctx.accounts.author.key();
    article.title = title;
    article.content = content;
    article.created_at = Clock::get()?.unix_timestamp;

    let author_profile = &mut ctx.accounts.author_profile;
    if author_profile.author == Pubkey::default() {
        author_profile.author = ctx.accounts.author.key();
    }
    author_profile.articles.push(article.key());

    Ok(())
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct CreateArticle<'info> {
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(
        init,
        payer = author,
        space = 8 + Article::INIT_SPACE,
        seeds = [b"article", author.key().as_ref(), title.as_bytes()],
        bump
    )]
    pub article: Account<'info, Article>,

    #[account(
        init,
        payer = author,
        space = 8 + AuthorProfile::INIT_SPACE,
        seeds = [b"author_profile", author.key().as_ref()],
        bump
    )]
    pub author_profile: Account<'info, AuthorProfile>,

    pub system_program: Program<'info, System>,
}
