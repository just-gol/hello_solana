use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct BurnTokenInfoArgs {
    pub name: String,
    pub symbol: String,
    pub url: String,
}
