use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Message {
    pub name: String,
    pub body: String,
    pub timestamp: u64,
}

#[account]
pub struct Board {
    pub messages: Vec<Message>,
}

impl Board {
    pub const SEED_BOARD: &'static str = "board";

    pub fn create_message(name: String, body: String) -> Result<Message> {
        Ok(Message {
            name,
            body,
            timestamp: Clock::get()?.unix_timestamp as u64,
        })
    }
}
