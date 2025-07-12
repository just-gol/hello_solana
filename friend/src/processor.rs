use crate::{instruction::SocialInstruction, state::*};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh1::try_from_slice_unchecked,
    clock::Clock,
    entrypoint::ProgramResult,
    example_mocks::solana_sdk::system_program,
    lamports,
    message::SanitizeMessageError,
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::Pack,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};
use solana_program::{msg, pubkey::Pubkey};
use spl_token::state::Mint;
pub struct Processor;

const MAX_FOLLOWER_SIZE: u16 = 200;
const PUBKEY_SIZE: usize = 32;
const USER_PROFILE_SIZE: usize = 6;
const U16_SIZE: usize = 2;

const USER_POST_SIZE: usize = 8;

impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        // 序列化指令枚举
        let instruction = SocialInstruction::try_from_slice(instruction_data)?;
        match instruction {
            SocialInstruction::InitializeUser { seed_type } => {
                Self::initialize_user(program_id, accounts, seed_type)
            }
            SocialInstruction::FollowUser { user_to_follow } => {
                Self::follow_user(accounts, user_to_follow)
            }
            SocialInstruction::UnfollowUser { user_to_follow } => {
                Self::unfollow_user(accounts, user_to_follow)
            }
            SocialInstruction::QueryFollowers => Self::query_followers(accounts),
            SocialInstruction::PostContent { content } => {
                Self::post_content(program_id, accounts, content)
            }
            SocialInstruction::QueryPosts => Self::query_posts(accounts),
        }
    }

    fn initialize_user(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        seed_type: String,
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let user_account_info = next_account_info(accounts_iter)?;
        let pda_account = next_account_info(accounts_iter)?;
        let system_program = next_account_info(accounts_iter)?;

        // 判断初始化关注,还是发帖
        let seed = match seed_type.as_str() {
            "profile" => "profile",
            "post" => "post",
            _ => return Err(ProgramError::InvalidArgument),
        };
        msg!("seed:{:?}", seed);

        // 生成pda
        let (pda, bump_seek) = Pubkey::find_program_address(
            &[user_account_info.key.as_ref(), seed.as_bytes()],
            program_id,
        );
        // 校验pda
        if pda != pda_account.key.clone() {
            return Err(ProgramError::InvalidArgument);
        }

        // 获取当前的租金信息（Rent 信息），用于后续创建账户时计算所需的最小 SOL。
        let rent = Rent::get()?;
        // 计算占用空间
        let space = match seed_type.as_str() {
            "profile" => computer_profile_number(MAX_FOLLOWER_SIZE),
            "post" => USER_POST_SIZE,
            _ => return Err(ProgramError::InvalidArgument),
        };

        let lamports = rent.minimum_balance(space);

        // 创建指令
        let create_account_ix = system_instruction::create_account(
            user_account_info.key,
            &pda,
            lamports,
            space as u64,
            program_id,
        );

        // 通过签名创建账户
        invoke_signed(
            &create_account_ix,
            &[
                user_account_info.clone(),
                pda_account.clone(),
                system_program.clone(),
            ],
            &[&[
                user_account_info.key.as_ref(),
                seed.as_bytes(),
                &[bump_seek],
            ]],
        )?;

        // 初始化数据
        match seed_type.as_str() {
            "profile" => {
                let user_profile = UserProfile::new();
                // 反序列化
                user_profile.serialize(&mut *pda_account.try_borrow_mut_data()?)?;
            }
            "post" => {
                let user_post = UserPost::new();
                user_post.serialize(&mut *pda_account.try_borrow_mut_data()?)?;
            }
            _ => return Err(ProgramError::InvalidArgument),
        }
        msg!("User initialized successfully.");
        Ok(())
    }

    fn follow_user(accounts: &[AccountInfo], user_to_follow: Pubkey) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let user_account_info = next_account_info(accounts_iter)?;
        msg!("user_account_info {:?}", user_account_info);

        let mut size: usize = 0;
        {
            let data = &user_account_info.data.borrow();
            let len = &data[..U16_SIZE];
            let pubkey_count = bytes_to_u16(len).unwrap();
            size = computer_profile_number(pubkey_count);
        }
        msg!("size is {}", size);
        let mut user_profile =
            UserProfile::try_from_slice(&user_account_info.data.borrow()[..size])?;
        msg!("user_profile {:?}", user_profile);

        user_profile.follow(user_to_follow);
        msg!("user_profile follow is {:?}", user_profile);

        user_profile.serialize(&mut *user_account_info.try_borrow_mut_data()?)?;
        msg!("User followed successfully.");
        Ok(())
    }

    fn unfollow_user(accounts: &[AccountInfo], user_to_follow: Pubkey) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let user_account_info = next_account_info(accounts_iter)?;
        msg!("user_account_info {:?}", user_account_info);
        let mut size: usize = 0;
        {
            let data = &user_account_info.data.borrow();
            let len: &[u8] = &data[..U16_SIZE];
            let pubkey_count = bytes_to_u16(len).unwrap();
            size = computer_profile_number(pubkey_count);
        }
        msg!("size is {}", size);
        let mut user_profile =
            UserProfile::try_from_slice(&user_account_info.data.borrow()[..size])?;
        msg!("user_profile {:?}", user_profile);

        user_profile.unfollow(user_to_follow);
        msg!("user_profile follow is {:?}", user_profile);

        user_profile.serialize(&mut *user_account_info.try_borrow_mut_data()?)?;
        msg!("User followed successfully.");
        Ok(())
    }

    fn query_followers(accounts: &[AccountInfo]) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let user_account_info = next_account_info(account_info_iter)?;
        msg!("pda:{:?}", user_account_info.key);

        let len = &user_account_info.data.borrow()[..U16_SIZE];
        let pubkey_count = bytes_to_u16(len).unwrap();
        msg!("let_number is {}", U16_SIZE);

        let size = computer_profile_number(pubkey_count);
        let user_profile = UserProfile::try_from_slice(&user_account_info.data.borrow()[..size])?;

        msg!("Followers:");
        msg!(" - {:?}", &user_profile);
        Ok(())
    }

    fn post_content(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        content: String,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let user_account_info = next_account_info(account_info_iter)?;
        let pda_account = next_account_info(account_info_iter)?;
        let post_pda_account = next_account_info(account_info_iter)?;
        let system_program = next_account_info(account_info_iter)?;
        // 获取时间戳
        let clock = Clock::get()?;
        let timestamp = clock.unix_timestamp as u64;
        let mut user_post = try_from_slice_unchecked::<UserPost>(&pda_account.data.borrow())?;
        user_post.add_post();

        user_post.serialize(&mut *pda_account.try_borrow_mut_data()?)?;

        let count = user_post.get_count();
        let (pda, bump_seed) = Pubkey::find_program_address(
            &[
                user_account_info.key.as_ref(),
                "post".as_bytes(),
                &[count as u8],
            ],
            program_id,
        );
        msg!("pda:{:?}", pda);

        let rent = Rent::get()?;
        let post = Post::new(content, timestamp);
        let space = borsh::to_vec(&post).unwrap().len();
        msg!("space:{}", space);

        let lamports = rent.minimum_balance(space);

        let create_account_ix = system_instruction::create_account(
            user_account_info.key,
            &pda,
            lamports,
            space as u64,
            program_id,
        );

        //通过签名创建账户
        invoke_signed(
            &create_account_ix,
            &[
                user_account_info.clone(),
                post_pda_account.clone(),
                system_program.clone(),
            ],
            &[&[
                user_account_info.key.as_ref(),
                "post".as_bytes(),
                &[count as u8],
                &[bump_seed],
            ]],
        )?;
        post.serialize(&mut *post_pda_account.try_borrow_mut_data()?)?;

        msg!("Post created successfully. {:?}", post);
        Ok(())
    }

    fn query_posts(accounts: &[AccountInfo]) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let pda_account = next_account_info(account_info_iter)?;
        let post_pda_account = next_account_info(account_info_iter)?;

        let user_post = try_from_slice_unchecked::<UserPost>(&pda_account.data.borrow())?;

        msg!("Posts:{:?}", user_post);

        let post = try_from_slice_unchecked::<Post>(&post_pda_account.data.borrow())?;
        msg!(" - {} at {}", post.content, post.timestamp);
        Ok(())
    }
}

fn bytes_to_u16(bytes: &[u8]) -> Option<u16> {
    if bytes.len() != 2 {
        return None;
    }
    let mut array = [0u8; 2];
    array.copy_from_slice(bytes);
    Some(u16::from_le_bytes(array))
}
fn computer_profile_number(pubkey_count: u16) -> usize {
    return USER_PROFILE_SIZE + pubkey_count as usize * PUBKEY_SIZE;
}
