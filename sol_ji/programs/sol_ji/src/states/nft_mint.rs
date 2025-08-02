use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct NftInfoArgs {
    pub name: String,
    pub symbol: String,
    pub url: String,
}

#[account]
#[derive(InitSpace)]
pub struct SbtNftCount {
    // sbt nft 总数
    pub count: u64,
}

impl SbtNftCount {
    pub fn increment(&mut self) {
        self.count += 1;
    }
}
