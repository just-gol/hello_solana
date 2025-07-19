use anchor_lang::prelude::*;

declare_id!("9uk7wEkBav2ZCRis2YcKuZdxsjBFsZA6wQbFHRy9MxCR");
mod instructions;
mod state;
use instructions::*;
#[program]
pub mod decentralized_message_board {
    use super::*;

    pub fn push_board(ctx: Context<PushBoard>, name: String, body: String) -> Result<()> {
        instructions::push_board(ctx, name, body)
    }
}
