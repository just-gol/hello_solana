use std::cmp::max;

use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StakeInfo {
    pub staker: Pubkey,           // 质押者的公钥
    pub nft_mint_account: Pubkey, // 质押的NFT的mint地址
    pub staked_at: u64,           // 质押时间
}

impl StakeInfo {
    pub fn new(staker: Pubkey, nft_mint: Pubkey) -> Self {
        let clock = Clock::get().unwrap();
        let staked_at = clock.epoch;
        Self {
            staker,
            nft_mint_account: nft_mint,
            staked_at,
        }
    }

    pub fn salvage_value(&self, amount: u64) -> u64 {
        let clock = Clock::get().unwrap();
        let now = clock.epoch;
        // 每一个epoch减2%
        // 下面代码是整形,小数会丢失
        // let epoch = now.checked_sub(self.staked_at)?;
        // let p = max(0, 100 - epoch.checked_mul(2)?).checked_div(100)?;

        let p = max(0, 100 - (now - self.staked_at) * 2) as f64 / 100.0;
        (amount as f64 * p) as u64
    }
}
