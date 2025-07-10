use borsh::{BorshDeserialize, BorshSerialize};

// 序列化,反序列化
#[derive(BorshDeserialize, BorshSerialize)]
pub struct GreetingAccount {
    pub count: u32,
}
