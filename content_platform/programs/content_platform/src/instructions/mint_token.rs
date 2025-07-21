use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, mint_to, Mint, MintTo, Token, TokenAccount, Transfer},
};

pub fn create_mint(ctx: Context<CreateMint>) -> Result<()> {
    msg!("create_mint receiver ata: {}", ctx.accounts.giver_ata.key());
    let mint_result = mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint_account.to_account_info(),
                to: ctx.accounts.giver_ata.to_account_info(),
                authority: ctx.accounts.treasury.to_account_info(),
            },
            &[&[b"treasury", &[ctx.bumps.treasury]]],
        ),
        10000,
    );
    if let Err(e) = mint_result {
        msg!("mint_to failed: {:?}", e);
        return Err(e);
    }
    Ok(())
}

pub fn reward_author(ctx: Context<RewardAuthor>) -> Result<()> {
    msg!(
        "reward_author receiver ata: {}",
        ctx.accounts.treasury_ata.key()
    );
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.giver_ata.to_account_info(),
                to: ctx.accounts.treasury_ata.to_account_info(),
                authority: ctx.accounts.giver_ata_wallet.to_account_info(),
            },
        ),
        5000,
    )?;
    Ok(())
}

pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
    msg!("withdraw receiver ata: {}", ctx.accounts.withdraw_ata.key());
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.treasury_ata.to_account_info(),
                to: ctx.accounts.withdraw_ata.to_account_info(),
                authority: ctx.accounts.treasury.to_account_info(),
            },
            &[&[b"treasury", &[ctx.bumps.treasury]]],
        ),
        2500,
    )?;
    Ok(())
}

#[derive(Accounts)]
pub struct CreateMint<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        seeds = [b"mint_account"],
        bump,
        mint::decimals = 2,
        mint::authority = treasury,
  )]
    pub mint_account: Account<'info, Mint>,

    // 创建作者创建一个ata账户
    #[account(
        init_if_needed,
        payer = payer,
        // 接收哪个 mint 所发出来的 token
        associated_token::mint = mint_account,
        // 谁拥有这个 ATA
        associated_token::authority = giver_ata_wallet,
    )]
    pub giver_ata: Account<'info, TokenAccount>,

    /// CHECK: Safe - not mutated
    #[account(
        seeds = [b"treasury"],
        bump,
    )]
    pub treasury: AccountInfo<'info>,

    /// CHECK: Safe - only used as ATA owner, not mutated or signed
    pub giver_ata_wallet: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct RewardAuthor<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        seeds = [b"mint_account"],
        bump,
    )]
    pub mint_account: Account<'info, Mint>,

    // 打款者的ata
    #[account(
      mut,
      constraint = giver_ata.owner == giver_ata_wallet.key(),  // 如果不添加这行,任意满足账户都可以打款
    )]
    pub giver_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub giver_ata_wallet: Signer<'info>,

    #[account(
        init_if_needed,
        seeds = [b"treasury_ata", treasury.key().as_ref(),mint_account.key().as_ref()],
        bump,
        payer = payer,
        token::mint = mint_account,
        token::authority = treasury,
    )]
    pub treasury_ata: Account<'info, TokenAccount>,

    /// CHECK: Safe - not mutated
    #[account(
      seeds = [b"treasury"],
      bump,
    )]
    pub treasury: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        seeds = [b"mint_account"],
        bump,
    )]
    pub mint_account: Account<'info, Mint>,

    // 创建作者创建一个ata账户
    #[account(
        mut,
        seeds = [b"treasury_ata", treasury.key().as_ref(),mint_account.key().as_ref()],
        bump,
        constraint = treasury_ata.owner == treasury.key(),
    )]
    pub treasury_ata: Account<'info, TokenAccount>,

    /// CHECK: Safe - not mutated
    #[account(
      seeds = [b"treasury"],
      bump,
    )]
    pub treasury: AccountInfo<'info>,

    #[account(
      init_if_needed,
      payer = payer,
      // 接收哪个 mint 所发出来的 token
      associated_token::mint = mint_account,
      // 谁拥有这个 ATA
      associated_token::authority = author_wallet,
  )]
    pub withdraw_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub author_wallet: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
