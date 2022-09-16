use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Transfer};

mod account_structs;
mod errors;
mod instructions;
mod state;

pub use account_structs::*;
pub use errors::*;
pub use instructions::*;
pub use state::*;

declare_id!("G8BgM1hwZjPWv8jkJhwpj1WKVneuUUuK9QKXDJxJtX2u");

#[program]
pub mod locked_voter {

    use super::*;

    pub fn init_pool(ctx: Context<InitPool>, bump: u8) -> Result<()> {
        instructions::init_pool::handler(ctx, bump)
    }

    pub fn init_entry(ctx: Context<InitEntry>, bump: u8) -> Result<()> {
        instructions::init_entry::handler(ctx, bump)
    }

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

        transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.source_tokens.to_account_info(),
                    to: ctx.accounts.escrow_tokens.to_account_info(),
                    authority: ctx.accounts.escrow_owner.to_account_info(),
                },
            ),
            1,
        )?;

        let escrow = &mut ctx.accounts.escrow;
        escrow.record_lock_event(next_escrow_started_at, next_escrow_ends_at)?;
        Ok(())
    }
}
