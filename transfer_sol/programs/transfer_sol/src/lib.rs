use anchor_lang::prelude::*;

declare_id!("5cYz3SRzKj3A8S9KeC4Xu7cFXCHkFvRLuafbZPKZ6Crb");
mod instructions;
use instructions::*;
#[program]
pub mod transfer_sol {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn transfer(ctx: Context<TransferSol>) -> Result<()> {
        instructions::transfer(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
