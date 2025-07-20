use anchor_lang::prelude::*;
#[account]
pub struct Config {
    // 设置管理员
    pub admin: Pubkey,
}

impl Config {
    pub const PREFIX: &'static str = "config";
    pub fn initialize(&mut self, admin: Pubkey) {
        self.admin = admin;
    }
}
