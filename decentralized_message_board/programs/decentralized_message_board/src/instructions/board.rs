use anchor_lang::prelude::*;

use crate::state::board::*;
pub fn push_board(ctx: Context<PushBoard>, name: String, body: String) -> Result<()> {
    // 创建用户
    let message = Board::create_message(name, body)?;
    let board = &mut ctx.accounts.board;
    board.messages.push(message);
    Ok(())
}

#[derive(Accounts)]
#[instruction(name: String, body: String)]
pub struct PushBoard<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 10000,
        seeds = [
            Board::SEED_BOARD.as_bytes(),
            user.key().as_ref(),
        ],
        bump,
    )]
    pub board: Account<'info, Board>,

    pub system_program: Program<'info, System>,
}
