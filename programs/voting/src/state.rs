use anchor_lang::{prelude::*, solana_program::pubkey::PUBKEY_BYTES};

#[account]
pub struct PollCount {
    /// The total number of proposal
    pub proposal_count: u64,
    /// Bump
    pub bump: u8,
}

impl PollCount {
    pub const LEN: usize = 8 + 1 + 1;
}

#[account]
pub struct Poll {
    pub id: u64,
    pub title: String,
    pub options: Vec<PollOption>, // max: 4 options
    pub options_count: u8,
    pub bump: u8,
}

impl Poll {
    pub const LEN: usize = 8 + 1 + 4 + (280 * 4) + 1 + 4 + (PollOption::LEN * 4) + 1 + 1;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct PollOption {
    pub id: u8,
    pub title: String,
    pub title_length: u8,
    pub votes: u64,
}

impl PollOption {
    const LEN: usize = 8 + 1 + 280 * 4 + 1 + 8;
}

#[account]
pub struct Vote {
    ///  The poll being voted on
    pub poll: Pubkey,
    /// The voter
    pub voter: Pubkey,
    /// The side of the vote taken
    pub option_selected: u8,
    /// Bump seed
    pub bump: u8,
}

impl Vote {
    pub const LEN: usize = 8 + 1 + PUBKEY_BYTES + 1 + 1;
}
