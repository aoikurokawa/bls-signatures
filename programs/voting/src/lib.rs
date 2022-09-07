use anchor_lang::prelude::*;

mod account_structs;
mod state;

use account_structs::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod voting {
    use super::*;

    pub fn create_poll(ctx: Context<CreatePoll>) -> Result<()> {
        Ok(())
    }
    pub fn vote_poll(ctx: Context<VotePoll>) -> Result<()> {
        Ok(())
    }
}
