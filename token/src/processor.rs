use borsh::BorshDeserialize;
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

use crate::instruction::TokenInstruction;

pub struct Processor;

impl Processor {
    pub fn process(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = TokenInstruction::try_from_slice(instruction_data)?;
        match instruction {
            TokenInstruction::CreateToken { decimals } => Self::create_token(accounts, decimals),
            TokenInstruction::Mint { amount } => Self::mint_token(accounts, amount),
        }
    }

    fn create_token(accounts: &[AccountInfo], decimals: u8) -> ProgramResult {
        // 生成account
        let accounts_iter = &mut accounts.iter();
        let mint_account = next_account_info(accounts_iter)?;
        let mint_authority = next_account_info(accounts_iter)?;
        let payer = next_account_info(accounts_iter)?;
        let rent_sysvar = next_account_info(accounts_iter)?;
        let system_program = next_account_info(accounts_iter)?;
        let token_program = next_account_info(accounts_iter)?;

        msg!("Creating mint account...");
        msg!("Mint account is {}", mint_account.key);

        let create_account_ix = &system_instruction::create_account(
            payer.key,
            mint_account.key,
            (Rent::get()?).minimum_balance(Mint::LEN),
            Mint::LEN as u64,
            token_program.key,
        );

        // invoke(
        //     create_account_ix,
        //     &[
        //         mint_account.clone(),
        //         payer.clone(),
        //         system_program.clone(),
        //         token_program.clone(),
        //     ],
        // )?;
        invoke(create_account_ix, &[payer.clone(), mint_account.clone()])?;

        // 初始化
        let mint_init_ix = &initialize_mint(
            token_program.key,
            mint_account.key,
            mint_authority.key,
            None,
            decimals,
        )?;

        msg!("initialize_mint account...");
        // invoke_signed(
        //     mint_init_ix,
        //     &[
        //         mint_account.clone(),
        //         rent_sysvar.clone(),
        //         token_program.clone(),
        //         mint_authority.clone(),
        //     ],
        //     &[],
        // )?;
        invoke_signed(
            mint_init_ix,
            &[mint_account.clone(), rent_sysvar.clone()],
            &[],
        )?;
        msg!("SPL Token Mint create success");

        Ok(())
    }
    fn mint_token(accounts: &[AccountInfo], amount: u64) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let mint_account = next_account_info(accounts_iter)?;
        let associated_token_account = next_account_info(accounts_iter)?;
        let rent_sysvar = next_account_info(accounts_iter)?;
        let payer = next_account_info(accounts_iter)?;
        let system_program = next_account_info(accounts_iter)?;
        let token_program: &AccountInfo<'_> = next_account_info(accounts_iter)?;
        let associated_token_program = next_account_info(accounts_iter)?;

        msg!("ATA is:{:?}", associated_token_account);

        if associated_token_account.lamports() == 0 {
            msg!("Creating assocated token account...");
            let create_ata_tx =
                &spl_associated_token_account::instruction::create_associated_token_account(
                    payer.key,
                    payer.key,
                    mint_account.key,
                    token_program.key,
                );
            invoke(
                create_ata_tx,
                &[
                    payer.clone(),
                    associated_token_account.clone(),
                    mint_account.clone(),
                    system_program.clone(),
                    token_program.clone(),
                    rent_sysvar.clone(),
                    associated_token_program.clone(),
                ],
            )?;
        }

        msg!("Minting {} tokens to ata...", amount);

        let mint_ix = &mint_to(
            token_program.key,
            mint_account.key,
            associated_token_account.key,
            payer.key,
            &[payer.key],
            amount,
        )?;

        invoke(
            mint_ix,
            &[
                mint_account.clone(),
                payer.clone(),
                associated_token_account.clone(),
                token_program.clone(),
            ],
        )?;

        msg!("Tokens Minted to ata success");

        Ok(())
    }
}
