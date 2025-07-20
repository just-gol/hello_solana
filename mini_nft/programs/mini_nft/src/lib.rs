use anchor_lang::prelude::*;

declare_id!("Gor99FaBkv4ysEAW9whwxmXFRAdH54RCdfXuidW25E8q");

mod instructions;
mod state;
use instructions::*;

#[program]
pub mod mini_nft {
    use super::*;

    // 设置管理员
    pub fn set_admin(ctx: Context<SetAdmin>, admin: Pubkey) -> Result<()> {
        instructions::set_admin(ctx, admin)
    }
    // 添加白名单
    pub fn add_whitelist(ctx: Context<AddWhitelist>, target: Pubkey) -> Result<()> {
        instructions::add_whitelist(ctx, target)
    }

    pub fn mint_nft(ctx: Context<MintNft>) -> Result<()> {
        instructions::mint_nft(ctx)
    }
}
