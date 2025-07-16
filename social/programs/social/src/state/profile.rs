use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct SolanaTwitterProfile {
    #[max_len(100)]
    pub display_name: String,
}

impl SolanaTwitterProfile {
    pub const PROFILE_SEED: &'static str = "profile";
}
