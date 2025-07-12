use borsh::{BorshDeserialize, BorshSerialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use std::{str::FromStr, time::Instant};

use solana_sdk::signature::read_keypair_file;

use solana_program::instruction::AccountMeta;

static PROFILE_SEED: &str = "profile";
static POST_SEED: &str = "post";

// 指令枚举（与合约保持一致）
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum SocialInstruction {
    InitializeUser { seed_type: String },
    FollowUser { user_to_follow: Pubkey },
    UnfollowUser { user_to_unfollow: Pubkey },
    PostContent { content: String },
    QueryFollowers,
    QueryPosts,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct UserProfile {
    pub data_len: u16,
    pub followers: Vec<Pubkey>, // 关注的用户列表
}
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Post {
    pub content: String,
    pub timestamp: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct UserPost {
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
impl UserProfile {
    pub fn new() -> Self {
        Self {
            data_len: 0,
            followers: vec![],
        }
    }
}

// 客户端核心逻辑
pub struct SocialClient {
    rpc_client: RpcClient,
    program_id: Pubkey,
}

impl SocialClient {
    pub fn new(rpc_url: &str, program_id: Pubkey) -> Self {
        let rpc_client = RpcClient::new(rpc_url.to_string());
        Self {
            rpc_client,
            program_id,
        }
    }

    pub fn initialize_user(
        &self,
        user_keypair: &Keypair,
        seed_type: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let pda = get_profile_pda(
            &self.program_id,
            &[user_keypair.pubkey().as_ref(), seed_type.as_bytes()],
        );
        let initialize_user_instruction = Instruction::new_with_borsh(
            self.program_id,
            &SocialInstruction::InitializeUser {
                seed_type: seed_type.to_string(),
            },
            vec![
                AccountMeta::new(user_keypair.pubkey(), true),
                AccountMeta::new(pda, false),
                AccountMeta::new(solana_sdk::system_program::id(), false),
            ],
        );
        self.send_instruction(user_keypair, vec![initialize_user_instruction])?;
        println!("User initialized successfully.");
        Ok(())
    }

    fn follow_user(
        &self,
        user_keypair: &Keypair,
        user_to_follow: Pubkey,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let pda = get_profile_pda(
            &self.program_id,
            &[user_keypair.pubkey().as_ref(), PROFILE_SEED.as_bytes()],
        );
        let instruction = Instruction::new_with_borsh(
            self.program_id,
            &SocialInstruction::FollowUser { user_to_follow },
            vec![AccountMeta::new(pda, false)],
        );
        self.send_instruction(user_keypair, vec![instruction])?;
        println!("User followed successfully.");
        Ok(())
    }

    fn unfollow_user(
        &self,
        payer: &Keypair,
        user_to_unfollow: Pubkey,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 获取pda
        let pda = get_profile_pda(&self.program_id, &[PROFILE_SEED.as_bytes()]);

        //创建指令
        let instuction = Instruction::new_with_borsh(
            self.program_id,
            &SocialInstruction::UnfollowUser { user_to_unfollow },
            vec![AccountMeta::new(pda, false)],
        );

        // 发送
        self.send_instruction(payer, vec![instuction])?;
        println!("User unfollowed successfully.");

        Ok(())
    }

    fn query_followers(&self, payer: &Keypair) -> Result<(), Box<dyn std::error::Error>> {
        let pda = get_profile_pda(
            &self.program_id,
            &[payer.pubkey().as_ref(), PROFILE_SEED.as_bytes()],
        );

        //创建指令
        let instuction = Instruction::new_with_borsh(
            self.program_id,
            &SocialInstruction::QueryFollowers,
            vec![AccountMeta::new(pda, false)],
        );

        self.send_instruction(payer, vec![instuction])?;
        println!("Followers queried successfully.");
        Ok(())
    }

    fn post_content(
        &self,
        payer: &Keypair,
        content: String,
        id: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let pda = get_profile_pda(
            &self.program_id,
            &[payer.pubkey().as_ref(), PROFILE_SEED.as_bytes()],
        );

        let post_pda = get_profile_pda(
            &self.program_id,
            &[payer.pubkey().as_ref(), POST_SEED.as_bytes(), &[id as u8]],
        );

        //创建指令
        let instuction = Instruction::new_with_borsh(
            self.program_id,
            &SocialInstruction::PostContent { content },
            vec![
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new(pda, false),
                AccountMeta::new(post_pda, false),
                AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
            ],
        );

        self.send_instruction(payer, vec![instuction])?;
        println!("Followers queried successfully.");
        Ok(())
    }

    pub fn query_posts(
        &self,
        user_keypair: &Keypair,
        id: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let post_pda = get_profile_pda(
            &self.program_id,
            &[
                user_keypair.pubkey().as_ref(),
                POST_SEED.as_bytes(),
                &[id as u8],
            ],
        );

        let pda = get_profile_pda(
            &self.program_id,
            &[user_keypair.pubkey().as_ref(), POST_SEED.as_bytes()],
        );

        let instruction = Instruction::new_with_borsh(
            self.program_id,
            &SocialInstruction::QueryPosts,
            vec![
                AccountMeta::new(pda, false),
                AccountMeta::new(post_pda, false),
            ],
        );

        self.send_instruction(user_keypair, vec![instruction])?;
        println!("Posts queried successfully.");
        Ok(())
    }

    fn send_instruction(
        &self,
        payer: &Keypair,
        instructions: Vec<Instruction>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let latest_blockhash = self.rpc_client.get_latest_blockhash()?;
        let transaction = Transaction::new_signed_with_payer(
            &instructions,
            Some(&payer.pubkey()),
            &[payer],
            latest_blockhash,
        );

        let signature = self.rpc_client.send_and_confirm_transaction(&transaction)?;
        println!("Transaction successful: {}", signature);
        Ok(())
    }
}
fn get_profile_pda(program_id: &Pubkey, seed: &[&[u8]]) -> Pubkey {
    let (pda, _bump) = Pubkey::find_program_address(seed, &program_id);
    println!("pad: {:?}", pda);
    return pda;
}
#[test]
fn test_friend() {
    let program_id = Pubkey::from_str("G4tuJ5f7n3mXzmrtRk5dnt9MJkF6JX4ou6bJQYe7kos6").unwrap();
    let client = SocialClient::new("http://127.0.0.1:8899", program_id);
    let payer = read_keypair_file("/home/lsy/.config/solana/id.json").expect("failed");

    // 初始化用户
    client.initialize_user(&payer, PROFILE_SEED);
    let user_to_follow = Pubkey::from_str("Azb6KFfzDpn8RCf94F8nYzAbS1LyiczxdsAtn3ryDGzR").unwrap();
    // 关注
    client.follow_user(&payer, user_to_follow);
    // 取消关注
    client.unfollow_user(&payer, user_to_follow);
    // 查询关注
    client.query_followers(&payer);

    // 初始化帖子
    client.initialize_user(&payer, POST_SEED);
    // 发布内容
    let post_content = "Hello Solana!".to_string();
    client.post_content(&payer, post_content, 1).unwrap();
    // 查询
    client.query_posts(&payer, 1).unwrap();
}

fn main() {}
