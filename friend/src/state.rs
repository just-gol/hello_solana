use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::msg;
use solana_program::pubkey::Pubkey;
// 定义结构体: 发帖,关注
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Post {
    // 发帖内容
    pub content: String,
    // 发帖时间
    pub timestamp: u64,
}

impl Post {
    pub fn new(content: String, timestamp: u64) -> Self {
        Self { content, timestamp }
    }
}
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct UserPost {
    // 发帖次数
    pub post_count: u64,
}

impl UserPost {
    pub fn new() -> Self {
        Self { post_count: 0 }
    }

    pub fn add_post(&mut self) {
        self.post_count += 1;
    }

    pub fn get_count(&self) -> u64 {
        self.post_count
    }
}
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct UserProfile {
    // 记录follower的长度
    // Vec<Pubkey> 是变长数据，反序列化时必须知道它的长度
    pub data_len: u16,
    //关注列表
    pub followers: Vec<Pubkey>,
}

impl UserProfile {
    pub fn new() -> Self {
        Self {
            data_len: 0,
            followers: vec![],
        }
    }

    pub fn follow(&mut self, user: Pubkey) {
        self.followers.push(user);
        self.data_len = self.followers.len() as u16;
        msg!("Followed successfully.");
        msg!("self is {:?}", self);
    }

    pub fn unfollow(&mut self, user: Pubkey) {
        self.followers.retain(|&x| x != user);
        self.data_len = self.followers.len() as u16;
    }
}
