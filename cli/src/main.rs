fn main() {
    println!("Hello, world!");
}
use std::str::FromStr;

// 7gLcZmqEbcgM6Bk5sjKuHmCUenfuQ2pSMYSLUTRcVzyW   部署的时候program id
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
    let program_id = Pubkey::from_str("7gLcZmqEbcgM6Bk5sjKuHmCUenfuQ2pSMYSLUTRcVzyW").unwrap();

    // 生成mint account
    let mint_account = Keypair::new();
    println!("mint account is {:?}", mint_account.pubkey().to_string());

    create_token(
        &rpc_client,
        &program_id,
        &payer,
        &mint_account,
        &payer.pubkey(),
        6,
    );
}

fn create_token(
    rpc_client: &RpcClient,
    program_id: &Pubkey,
    payer: &Keypair,
    mint_account: &Keypair,
    mint_authority: &Pubkey,
    decimals: u8,
) -> Result<(), Box<dyn std::error::Error>> {
    let instrution_data = borsh::to_vec(&TokenInstruction::CreateToken { decimals }).unwrap();
    let accounts = vec![
        AccountMeta::new(mint_account.pubkey(), true),
        AccountMeta::new_readonly(*mint_authority, false),
        AccountMeta::new_readonly(payer.pubkey(), false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
        AccountMeta::new_readonly(spl_token::id(), false),
    ];

    let token_instruction = Instruction {
        program_id: *program_id,
        accounts,
        data: instrution_data,
    };

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
