use anchor_lang::prelude::*;

#[account]
pub struct Lock {
    /// Base account used to generate signer seeds
    pub base: Pubkey,
    /// Bump seed
    pub bump: u8,
    /// Mint of the token that must be locked in the [Lock]
    pub token_mint: Pubkey,
}

#[account]
pub struct Escrow {
    /// The [Lock] that this [Escrow] is part of
    pub lock: Pubkey,

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
