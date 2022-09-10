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
#[derive(Debug)]
pub struct Poll {
    /// The unique ID of the poll, auto-incremented
    pub index: u64,
    /// Bump seed
    pub bump: u8,

    /// The public key of proposer
    pub proposer: Pubkey,

    /// Current number of votes in favor of this proposal
    pub for_votes: u64,
    /// Current number of votes in opposition to this proposal
    pub against_votes: u64,

    /// Title of the poll
    pub title: String,

    /// Link to description of the proposal
    pub desctiption_link: String,
}

impl Poll {
    pub const LEN: usize = 8 + 8 + 1 + PUBKEY_BYTES + 8 + 8;
}

// #[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
// pub struct PollOption {
//     pub id: u8,
//     pub title: String,
//     pub title_length: u8,
//     pub votes: u64,
// }

// impl PollOption {
//     const LEN: usize = 8 + 1 + 280 * 4 + 1 + 8;
// }

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
