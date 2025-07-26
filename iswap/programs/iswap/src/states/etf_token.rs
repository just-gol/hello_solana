use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct EtfToken {
    pub mint_account: Pubkey,
    pub creator: Pubkey,
    pub create_at: i64,
    #[max_len(50)]
    pub descriptor: String,
    #[max_len(10)]
    pub assets: Vec<EftAsset>,
}

impl EtfToken {
    pub const SEED_PREFIX: &'static str = "ETF_token_v3";
    pub const TOKEN_DECIMALS: u8 = 9;
}

// 资产
#[account]
#[derive(InitSpace)]
pub struct EftAsset {
    // token
    pub token: Pubkey,
    // 权重
    pub weight: u16,
}
