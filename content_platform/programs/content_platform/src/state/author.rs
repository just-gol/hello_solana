use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct AuthorProfile {
    pub author: Pubkey,
    #[max_len(100)]
    pub articles: Vec<Pubkey>,
}
