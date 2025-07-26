use anchor_lang::prelude::*;

declare_id!("9Hy6ivQ1LU4ZvAXKLUZZvCKDPCQfdKYHhjzMre9AnD2r");
pub mod instructions;
use instructions::*;
pub mod states;
#[program]
pub mod iswap {
    use super::*;

    pub fn eft_create(ctx: Context<EtfTokenCreate>, args: EtfTokenArgs) -> Result<()> {
        instructions::eft_token_create(ctx, args)
    }
}
