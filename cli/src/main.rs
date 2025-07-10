fn main() {
    println!("Hello, world!");
}
use std::{str::FromStr, vec};

// 7gLcZmqEbcgM6Bk5sjKuHmCUenfuQ2pSMYSLUTRcVzyW   部署的时候program id
// G4tuJ5f7n3mXzmrtRk5dnt9MJkF6JX4ou6bJQYe7kos6
use borsh::{BorshDeserialize, BorshSerialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
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
fn test_fn() {
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
    let accounts = vec![
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
