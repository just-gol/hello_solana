use anchor_lang::prelude::*;
declare_id!("8XmSWYV8TjhJhTYBDBmk1fRGgupkX8cfPqdUAegCiizw");

#[program]
pub mod todo {

    use super::*;

    pub fn initialize(ctx: Context<CreateTodoAccount>, title: String) -> Result<()> {
        let todo_account = &mut ctx.accounts.todo_account;
        todo_account.owner = *ctx.accounts.user.key;
        todo_account.title = title;
        todo_account.item = Vec::new();
        Ok(())
    }

    pub fn add_item(ctx: Context<ModifyAccount>, item: String) -> Result<()> {
        let todo_account = &mut ctx.accounts.todo_account;
        require!(
            todo_account.owner == *ctx.accounts.user.key,
            ErrorCode::Unauthorized
        );
        todo_account.item.push(item);
        Ok(())
    }

    pub fn remove_item(ctx: Context<ModifyAccount>, index: u64) -> Result<()> {
        let todo_account = &mut ctx.accounts.todo_account;
        require!(
            todo_account.owner == *ctx.accounts.user.key,
            ErrorCode::Unauthorized
        );
        require!(
            todo_account.item.len() > index as usize,
            ErrorCode::IndexOutOfBounds
        );
        todo_account.item.remove(index as usize);
        Ok(())
    }

    pub fn query_item(ctx: Context<ModifyAccount>, index: u64) -> Result<String> {
        let todo_account = &mut ctx.accounts.todo_account;
        require!(
            todo_account.owner == *ctx.accounts.user.key,
            ErrorCode::Unauthorized
        );
        require!(
            todo_account.item.len() > index as usize,
            ErrorCode::IndexOutOfBounds
        );
        let item = todo_account
            .item
            .get(index as usize)
            .ok_or(ErrorCode::IndexOutOfBounds)?;
        Ok(item.clone())
    }
}

#[account]
pub struct TodoAccount {
    pub owner: Pubkey,
    pub title: String,
    pub item: Vec<String>,
}

#[derive(Accounts)]
pub struct ModifyAccount<'info> {
    #[account(mut)]
    pub todo_account: Account<'info, TodoAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct CreateTodoAccount<'info> {
    #[account(init, payer = user, space = 8 + 32 + 4 + 100)]
    pub todo_account: Account<'info, TodoAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized operation.")]
    Unauthorized,

    #[msg("Index out of bounds.")]
    IndexOutOfBounds,
}
