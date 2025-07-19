use anchor_lang::prelude::*;

declare_id!("FAByA4r6tHxtyQLtWvuk8SQgmEe73miqVfrJjzxmmzRG");

mod instructions;
use instructions::*;
mod state;

#[program]
pub mod multi_user_counter {
    use super::*;

    pub fn increment(ctx: Context<AccountCounter>, author: Pubkey) -> Result<()> {
        instructions::increment(ctx, author)
    }
    pub fn decriment(ctx: Context<AccountCounter>, author: Pubkey) -> Result<()> {
        instructions::decriment(ctx, author)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
