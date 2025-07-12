use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

// 定义指令
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum SocialInstruction {
    // 初始化,账户
    InitializeUser { seed_type: String },
    // 关注
    FollowUser { user_to_follow: Pubkey },
    // 取消关注
    UnfollowUser { user_to_follow: Pubkey },
    // 发帖
    PostContent { content: String },
    // 查询关注
    QueryFollowers,
    // 查询发帖
    QueryPosts,
}
