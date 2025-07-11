use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

// 序列化,反序列化
#[derive(BorshDeserialize, BorshSerialize)]
pub enum ScoreInstruction {
    /// 初始化账户
    InitScore,
    /// 增加分数
    AddScore { amount: u64 },
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ScoreAccount {
    pub player: Pubkey,
    pub score: u64,
}
