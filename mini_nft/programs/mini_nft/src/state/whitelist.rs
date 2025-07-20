use anchor_lang::prelude::*;

#[account]
pub struct Whitelist {
    pub mint_start_time: u64,
    pub mint_end_time: u64,
    pub target: Pubkey,
    pub is_minted: bool,
}

impl Whitelist {
    pub const PREFIX: &'static str = "whitelist";

    pub fn initialize(&mut self, mint_start_time: u64, mint_end_time: u64, target: Pubkey) {
        self.mint_start_time = mint_start_time;
        self.mint_end_time = mint_end_time;
        self.target = target;
        self.is_minted = false;
    }
}
