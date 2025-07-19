use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, MintTo, Token, TokenAccount},
};
pub fn transfer(ctx: Context<TransferSol>) -> Result<()> {
    // 首先mint
    let signer_seeds: &[&[&[u8]]] = &[&[b"mint", &[ctx.bumps.mint_account]]];

    let mint_tx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.mint_account.to_account_info(),
            to: ctx.accounts.ata1.to_account_info(),
            authority: ctx.accounts.mint_account.to_account_info(),
        },
        signer_seeds,
    );

    token::mint_to(mint_tx, 1000000)?;

    // 转账
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.ata1.to_account_info(),
                to: ctx.accounts.ata2.to_account_info(),
                authority: ctx.accounts.from.to_account_info(),
            },
        ),
        5000,
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct TransferSol<'info> {
    #[account(mut)]
    pub from: Signer<'info>,

    #[account(
      init_if_needed,
      payer= from,
      seeds =[b"mint"],
      bump,
      mint::decimals = 2,
      mint::authority = mint_account.key(),
    )]
    pub mint_account: Account<'info, Mint>,

    // 用户的主ATA
    #[account(
      init_if_needed,
      payer = from,
      associated_token::mint = mint_account,
      associated_token::authority = from,
    )]
    pub ata1: Account<'info, TokenAccount>,

    // 用户的另外ATA
    #[account(
      init_if_needed,
      payer = from,
      associated_token::mint = mint_account,
      associated_token::authority = second_owner,
    )]
    pub ata2: Account<'info, TokenAccount>,

    /// CHECK: just a pubkey used for ATA2
    pub second_owner: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}
