use std::str::FromStr;

use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::{read_keypair, read_keypair_file};
use solana_sdk::signer::Signer;
use solana_sdk::system_instruction;
use solana_sdk::transaction::Transaction;
use solana_sdk::{pubkey::Pubkey, signature};
// ipwacQbTTWbfu6VnUyxdDDG8rwGvRjpqenzC6M2Bdys
// 6C4mMZegCHZJMzbhhdCLDSigSJjjCCdvRKkJvfAD8Yy7
fn main() {
    // 创建solana连接
    let rpc_url = "http://127.0.0.1:8899";
    // 建立连接
    let client = RpcClient::new(rpc_url);
    // 获取公钥 发送方
    let sender = read_keypair_file("/home/lsy/.config/solana/id.json").unwrap();
    // 接收方
    let recipient = Pubkey::from_str("6C4mMZegCHZJMzbhhdCLDSigSJjjCCdvRKkJvfAD8Yy7").unwrap();

    // 转账数量
    let amount = 1 * 1_000_000_000;

    // 转账指令
    let transfer = system_instruction::transfer(&sender.pubkey(), &recipient, amount);

    // 创建交易
    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let transaction = Transaction::new_signed_with_payer(
        &[transfer],
        Some(&sender.pubkey()),
        &[sender],
        recent_blockhash,
    );

    // 发送
    let result = client.send_and_confirm_transaction(&transaction);
    match result {
        Ok(signature) => println!("转账成功,签名:{}", signature),
        Err(err) => eprintln!("转账失败:{}", err),
    }

    // match client.request_airdrop(&account_pubkey, amount) {
    //     Ok(signature) => println!("空投成功,签名:{}", signature),
    //     Err(err) => eprintln!("空投失败:{}", err),
    // }

    // 获取账户余额
    // match client.get_balance(&account_pubkey) {
    //     Ok(balance) => println!("账户余额:{} lamports", balance),
    //     Err(err) => eprintln!("获取账户余额失败:{}", err),
    // }
}
