use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

mod account_structs;
mod state;

pub use account_structs::*;
pub use state::*;

declare_id!("G8BgM1hwZjPWv8jkJhwpj1WKVneuUUuK9QKXDJxJtX2u");

#[program]
pub mod locked_voter {
    use super::*;

    pub fn new_locker(ctx: Context<NewLock>, bump: u8) -> Result<()> {
        let locker_acc = &mut ctx.accounts.locker;

        let locker = Locker {
            base: ctx.accounts.base.key(),
            bump,
            token_mint: ctx.accounts.token_mint.key(),
        };

        locker_acc.set_inner(locker);

        Ok(())
    }

    pub fn new_escrow(ctx: Context<NewEscrow>, bump: u8) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;

        let new_escrow = Escrow {
            locker: ctx.accounts.locker.key(),
            owner: ctx.accounts.escrow_owner.key(),
            bump,
            tokens: anchor_spl::associated_token::get_associated_token_address(
                &escrow.key(),
                &ctx.accounts.locker.token_mint,
            ),
            amount: 1,
            escrow_started_at: 0,
            escrow_ends_at: 0,
        };

        escrow.set_inner(new_escrow);

        Ok(())
    }

    pub fn lock(ctx: Context<Lock>) -> Result<()> {
        let next_escrow_started_at = Clock::get()?.unix_timestamp;
        let next_escrow_ends_at = Escrow::STAKE_DURATION
            .checked_add(next_escrow_started_at)
            .unwrap_or(next_escrow_started_at);

        let escrow = &mut ctx.accounts.escrow;
        escrow.record_lock_event(next_escrow_started_at, next_escrow_ends_at)?;
        Ok(())
    }
}
