use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize)]
pub enum TokenInstruction {
    CreateToken { decimals: u8 },
    Mint { amount: u64 },
}
