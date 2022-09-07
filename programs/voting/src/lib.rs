use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod voting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        println!("Hello ");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[account]
pub struct PollCount {
    pub is_initialized: bool,
    pub count: u8,
    pub bump: u8,
}

#[account]
pub struct Poll {
    pub is_initialized: bool,
    pub id: u8,
    pub title: String,
    pub title_length: u8,
    pub options: Vec<PollOption>,
    pub options_count: u8,
    pub bump: u8,
}

#[account]
pub struct PollOption {
    pub id: u8,
    pub title: String,
    pub title_length: u8,
    pub votes: u64,
}
