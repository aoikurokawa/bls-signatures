use anchor_lang::prelude::*;

mod account_structs;
mod state;

use account_structs::*;
use state::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod voting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, bump: u8) -> Result<()> {
        let count_data = PollCount {
            proposal_count: 0,
            bump,
        };

        ctx.accounts.count_data.set_inner(count_data);
        Ok(())
    }

    pub fn create_poll(ctx: Context<CreatePoll>) -> Result<()> {
        let count_data = &mut ctx.accounts.count_data;

        Ok(())
    }

    pub fn vote_poll(ctx: Context<VotePoll>) -> Result<()> {
        Ok(())
    }
}
