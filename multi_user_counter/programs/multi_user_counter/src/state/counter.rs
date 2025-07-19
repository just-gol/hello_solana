use anchor_lang::prelude::*;

#[account]
pub struct Counter {
    pub count: u64,
}

impl Counter {
    pub const PREFIX: &'static str = "counter";
    pub fn increment(&mut self) {
        self.count += 1;
    }

    pub fn decriment(&mut self) {
        self.count -= 1;
    }
}
