use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::{self, ProgramResult},
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};

use crate::instruction::{ScoreAccount, ScoreInstruction};

pub struct Processor;

impl Processor {
    pub fn process(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        // 反序列化
        let data = ScoreInstruction::try_from_slice(instruction_data)?;
        match data {
            ScoreInstruction::InitScore => {
                // 获取account
                let accounts_iter = &mut accounts.iter();
                let account = next_account_info(accounts_iter)?;
                if account.owner != _program_id {
                    return Err(ProgramError::IncorrectProgramId);
                }
                let mut score_account = ScoreAccount::try_from_slice(&account.data.borrow())?;
                score_account.player = *account.key;
                score_account.score = 0;

                // 序列化
                score_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
                msg!("init success,score_account player:{}", score_account.player);
            }
            ScoreInstruction::AddScore { amount } => {
                let accounts_iter = &mut accounts.iter();
                let account = next_account_info(accounts_iter)?;
                let mut score_account = ScoreAccount::try_from_slice(&account.data.borrow())?;
                // check
                if *account.key != score_account.player {
                    return Err(ProgramError::InvalidAccountData);
                }
                score_account.score += amount;
                score_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
                msg!(
                    "modify success, amount:{}, score:{}",
                    amount,
                    score_account.score
                );
            }
        }
        Ok(())
    }
}
