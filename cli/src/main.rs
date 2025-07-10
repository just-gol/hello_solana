fn main() {
    println!("Hello, world!");
}
use std::{str::FromStr, vec};

// 7gLcZmqEbcgM6Bk5sjKuHmCUenfuQ2pSMYSLUTRcVzyW   部署的时候program id
// G4tuJ5f7n3mXzmrtRk5dnt9MJkF6JX4ou6bJQYe7kos6
use borsh::{BorshDeserialize, BorshSerialize};
use solana_client::rpc_client::RpcClient;
use solana_program::example_mocks::solana_sdk::system_instruction;
use solana_sdk::{
    account::Account,
    config::program,
    instruction::{AccountMeta, Instruction},
    lamports,
    pubkey::Pubkey,
    signature::{Keypair, read_keypair_file},
    signer::Signer,
    sysvar::{self, last_restart_slot},
    transaction::Transaction,
};
use spl_associated_token_account_client::address::get_associated_token_address;

#[derive(BorshDeserialize, BorshSerialize)]
pub enum TokenInstruction {
    CreateToken { decimals: u8 },
    Mint { amount: u64 },
}

#[test]
fn test_fn_token() {
    let rpc_client = RpcClient::new("http://127.0.0.1:8899".to_string());
    // 付款

    let payer = read_keypair_file("/home/lsy/.config/solana/id.json").expect("failed");
    let program_id = Pubkey::from_str("G4tuJ5f7n3mXzmrtRk5dnt9MJkF6JX4ou6bJQYe7kos6").unwrap();

    // 生成mint account
    let mint_account = Keypair::new();
    println!("mint account is {:?}", mint_account.pubkey().to_string());

    _ = create_token(
        &rpc_client,
        &program_id,
        &payer,
        &mint_account,
        &payer.pubkey(),
        6,
    );
    _ = mint(&rpc_client, &program_id, &payer, &mint_account, 100_000_000);
}

fn create_token(
    rpc_client: &RpcClient,
    program_id: &Pubkey,
    payer: &Keypair,
    mint_account: &Keypair,
    mint_authority: &Pubkey,
    decimals: u8,
) -> Result<(), Box<dyn std::error::Error>> {
    // 将指令序列化为字节数组
    let instrution_data = borsh::to_vec(&TokenInstruction::CreateToken { decimals }).unwrap();

    // 构建账户元数据
    let accounts: Vec<AccountMeta> = vec![
        AccountMeta::new(mint_account.pubkey(), true),
        AccountMeta::new_readonly(*mint_authority, false),
        AccountMeta::new_readonly(payer.pubkey(), false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
        AccountMeta::new_readonly(spl_token::id(), false),
    ];

    // 构建指令
    let token_instruction = Instruction {
        program_id: *program_id,
        accounts,
        data: instrution_data,
    };

    // 发送交易
    let latest_blockhash = rpc_client.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(
        &[token_instruction],
        Some(&payer.pubkey()),
        &[payer, mint_account],
        latest_blockhash,
    );

    let r = rpc_client.send_and_confirm_transaction(&tx)?;
    println!("{:?}", r);

    println!("create token success");

    Ok(())
}

fn mint(
    rpc_client: &RpcClient,
    program_id: &Pubkey,
    payer: &Keypair,
    mint_account: &Keypair,
    amount: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    // 创建ATA账户
    let ata = get_associated_token_address(&payer.pubkey(), &mint_account.pubkey());
    println!("ata is {:?}", ata.to_string());

    let instrution_data = borsh::to_vec(&TokenInstruction::Mint { amount }).unwrap();

    let accounts = vec![
        AccountMeta::new(mint_account.pubkey(), true),
        AccountMeta::new(ata, false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        AccountMeta::new(payer.pubkey(), true),
        AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(spl_associated_token_account::id(), false),
    ];

    let token_instruction = Instruction {
        program_id: *program_id,
        accounts,
        data: instrution_data,
    };

    // 发送交易
    let latest_blockhash = rpc_client.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(
        &[token_instruction],
        Some(&payer.pubkey()),
        &[payer, mint_account],
        latest_blockhash,
    );
    let r = rpc_client.send_and_confirm_transaction(&tx)?;
    println!("{:?}", r);
    println!("create mint success");
    Ok(())
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct GreetingAccount {
    pub count: u32,
}

#[test]
fn test_fn_count() {
    let rpc_client = RpcClient::new("http://localhost:8899".to_string());
    let payer = read_keypair_file("/home/lsy/.config/solana/id.json").expect("failed");
    let program_id = Pubkey::from_str("6dxkWb9FndEDrByTZnPjEKqwYDc19P8qEt9CX2Yj7Ysc").unwrap();
    let greeting_account = Keypair::new();
    let space = 4;
    let lamports = rpc_client
        .get_minimum_balance_for_rent_exemption(space)
        .unwrap();

    // 类似anchor的#[account(init)]
    // create_account 是为了创建一个“专门用于存储你自定义数据（如 GreetingAccount）”的账户
    let create_tx = system_instruction::create_account(
        &payer.pubkey(),            // 👈 谁来付款（payer）
        &greeting_account.pubkey(), // 👈 要创建的账户的地址（一般是 Keypair::new().pubkey()）
        lamports,                   // 👈 给这个账户转多少 SOL（需要覆盖租金）
        space as u64,               // 👈 分配多少空间（单位：字节）
        &program_id,                // 👈 这个账户归哪个程序控制（比如你的合约ID）
    );

    // d调用hello 指令
    let greet_ix = Instruction {
        program_id,
        accounts: vec![AccountMeta::new(greeting_account.pubkey(), false)],
        data: vec![],
    };

    //构造发送交易
    let recent_blockhash = rpc_client.get_latest_blockhash();
    let tx = Transaction::new_signed_with_payer(
        &[create_tx, greet_ix],
        Some(&payer.pubkey()),
        &[&payer, &greeting_account],
        recent_blockhash.unwrap(),
    );
    let sig = rpc_client.send_and_confirm_transaction(&tx);
    println!("Transaction sent: {}", sig.unwrap());

    let acc = rpc_client.get_account(&greeting_account.pubkey()).unwrap();
    let result = GreetingAccount::try_from_slice(&acc.data).unwrap();
    println!("Greeting count is: {}", result.count);
}
