use crate::state::author::*;
use anchor_lang::prelude::*;

pub fn view_articles(ctx: Context<ViewArticles>, offset: u64, limit: u64) -> Result<()> {
    let profile = &ctx.accounts.author_profile;

    let start = offset as usize;
    let end = std::cmp::min(start + limit as usize, profile.articles.len());

    let paged_articles = &profile.articles[start..end];

    msg!("Paged Articles:");
    for pubkey in paged_articles {
        msg!(" - {}", pubkey);
    }

    Ok(())
}
#[derive(Accounts)]
pub struct ViewArticles<'info> {
    /// CHECK: This is only used for reading, not modifying data
    pub author: AccountInfo<'info>,

    #[account(
      seeds = [b"author_profile", author.key().as_ref()],
      bump,
  )]
    pub author_profile: Account<'info, AuthorProfile>,
}
