use anchor_lang::{prelude::*, solana_program::pubkey::PUBKEY_BYTES};

#[account]
pub struct PollCount {
    /// The total number of proposal
    pub proposal_count: u8,
    /// Bump
    pub bump: u8,
}

impl PollCount {
    pub const LEN: usize = 8 + 1 + 1;
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
