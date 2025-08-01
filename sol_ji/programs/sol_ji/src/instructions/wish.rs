use anchor_lang::prelude::*;

use crate::states::{PublishWish, UserBurnInfo, WishLike, WishUser};
// 创建用户
pub fn create_wish_user(ctx: Context<CreateWishUser>) -> Result<()> {
    let wish_user = WishUser::new();
    ctx.accounts.wish_user.set_inner(wish_user);
    msg!("create wish user success");
    Ok(())
}

// 许愿 value是扣除功德值
pub fn wish(
    ctx: Context<CreateWish>,
    content: String,
    value: u64,
    is_anonymous: bool,
) -> Result<()> {
    let wish_user = &mut ctx.accounts.wish_user;
    wish_user.check_is_free();
    wish_user.update_user_wish_count();

    // 扣除功德值
    if wish_user.daily_count > 3 {
        if ctx.accounts.user_burn_info.merit_value < value {
            return Err(WishCode::Insufficient.into());
        }
        ctx.accounts.user_burn_info.deduction(value);
    }
    let publish_wish = if is_anonymous {
        // 匿名
        PublishWish::new(Pubkey::default(), content)
    } else {
        PublishWish::new(ctx.accounts.authority.key(), content)
    };
    ctx.accounts.publish_wish.set_inner(publish_wish);
    Ok(())
}

pub fn create_like(ctx: Context<CreateLike>) -> Result<()> {
    let publish_wish = &mut ctx.accounts.publish_wish;
    publish_wish.like_count += 1;

    ctx.accounts.wish_like.set_inner(WishLike::new(
        ctx.accounts.authority.key(),
        ctx.accounts.publish_wish.key(),
    ));
    Ok(())
}

#[derive(Accounts)]
pub struct CreateWishUser<'info> {
    #[account(
      init,
      payer = authority,
      space = 8 + WishUser::INIT_SPACE,
      seeds = [b"wish_user",authority.key().as_ref()], 
      bump
    )]
    pub wish_user: Account<'info, WishUser>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateWish<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
      init,
      payer = authority,
      space = 8 + PublishWish::INIT_SPACE,
      seeds = [b"wish",wish_user.key().as_ref(),(wish_user.total_count+1).to_string().as_bytes()], 
      bump
    )]
    pub publish_wish: Account<'info, PublishWish>,

    #[account(
      mut,
      seeds = [b"wish_user",authority.key().as_ref()], 
      bump
    )]
    pub wish_user: Account<'info, WishUser>,

    // 功德值->在这个账户中
    #[account(
      mut,
      seeds = [b"user_burn_info",authority.key().as_ref()],
      bump
    )]
    pub user_burn_info: Account<'info, UserBurnInfo>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateLike<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    // 许愿
    #[account(mut)]
    pub publish_wish: Account<'info, PublishWish>,

    // 用户
    #[account(
      mut,
      seeds = [b"wish_user",authority.key().as_ref()], 
      bump
    )]
    pub wish_user: Account<'info, WishUser>,

    // 点赞
    #[account(
      init,
      payer = authority,
      space = 8 + WishLike::INIT_SPACE,
      seeds = [b"wish_like",wish_user.key().as_ref(),publish_wish.key().as_ref()], 
      bump
    )]
    pub wish_like: Account<'info, WishLike>,

    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum WishCode {
    #[msg("insufficient merit value")]
    Insufficient,
}
