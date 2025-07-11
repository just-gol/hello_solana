use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::{self, ProgramResult},
    msg,
    program::{invoke, invoke_signed},
    program_pack::Pack,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use spl_token::{
    instruction::{initialize_mint, mint_to},
    state::Mint,
};

use crate::instruction::GreetingAccount;

pub struct Processor;

impl Processor {
    pub fn process(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let account = next_account_info(accounts_iter)?;
        let mut data = GreetingAccount::try_from_slice(&account.data.borrow())?;
        data.count += 1;
        // 把你 Rust 结构体 data 的内容序列化（转成字节），写入到这个账户的 data 区域里
        data.serialize(&mut &mut account.data.borrow_mut()[..])?;
        msg!("count:{}", data.count);
        Ok(())
    }
}
