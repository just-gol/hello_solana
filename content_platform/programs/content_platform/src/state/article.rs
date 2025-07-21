use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Article {
    pub author: Pubkey,
    #[max_len(50)]
    pub title: String,
    #[max_len(100)]
    pub content: String,
    pub created_at: i64,
}

impl Article {
    pub const PREFIX: &'static str = "article";

    pub fn new(author: Pubkey, title: String, content: String) -> Self {
        Self {
            author,
            title,
            content,
            created_at: Clock::get().unwrap().unix_timestamp,
        }
    }
}
