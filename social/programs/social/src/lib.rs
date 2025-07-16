use anchor_lang::prelude::*;

declare_id!("Ek9GNFGdLJRNVoJhzyF5WsuxUMobUSKz6CzM1o9fPzFk");
pub mod instructions;
pub mod state;
use instructions::*;

#[program]
pub mod social {
    use super::*;

    pub fn create_profile(ctx: Context<CreateProfile>, display_name: String) -> Result<()> {
        instructions::create_profile(ctx, display_name)
    }
}
