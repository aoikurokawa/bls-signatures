use anchor_lang::solana_program::pubkey::PUBKEY_BYTES;
use crate::*;

pub const STAKE_ENTRY_PREFIX: &str = "my_khe_stake_entry";

#[account]
pub struct StakePool {
    pub bump: u8,
    pub authority: Pubkey,
    pub requires_creator: Pubkey,
    pub total_staked: u32,
}

#[account]
pub struct StakeEntry {
    pub bump: u8,
    pub pool: Pubkey,
    pub amount: u64,
    pub original_mint: Pubkey,
    pub stake_mint: Option<Pubkey>,
}

#[account]
pub struct Locker {
    /// Base account used to generate signer seeds
    pub base: Pubkey,
    /// Bump seed
    pub bump: u8,
    /// Mint of the token that must be locked in the [Lock]
    pub token_mint: Pubkey,
}

impl Locker {
    pub const LEN: usize = PUBKEY_BYTES + 1 + PUBKEY_BYTES;
}

#[account]
pub struct Escrow {
    /// The [Lock] that this [Escrow] is part of
    pub locker: Pubkey,

    /// The key of the account that is authorized to stake into/withdraw from this [Escrow]
    pub owner: Pubkey,

    /// Bump seed
    pub bump: u8,

    /// The tokens account holding the escrow tokens
    pub tokens: Pubkey,

    /// Amount of tokens staked
    pub amount: u64,

    /// When the [Escrow::owner] started their escrow
    pub escrow_started_at: i64,

    /// When the escrow unlocks; i.e. the [Escrow::owner] is scheduled to be allowed to withdraw their tokens
    pub escrow_ends_at: i64,
}

impl Escrow {
    pub const LEN: usize = PUBKEY_BYTES + PUBKEY_BYTES + 1 + PUBKEY_BYTES + 8 + 8 + 8;
    pub const STAKE_DURATION: i64 = 31556926; // 1 year

    pub fn record_lock_event(
        &mut self,
        next_escrow_started_at: i64,
        next_escrow_ends_at: i64,
    ) -> Result<()> {
        self.escrow_started_at = next_escrow_started_at;
        self.escrow_ends_at = next_escrow_ends_at;
        Ok(())
    }
}

pub fn get_stake_seed(supply: u64, user: Pubkey) -> Pubkey {
    if supply > 1 {
        user
    } else {
        Pubkey::default()
    }
}
