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

    pub fn create_poll(
        ctx: Context<CreatePoll>,
        title: String,
        options: Vec<PollOption>,
        bump: u8,
    ) -> Result<()> {
        let count_data = &mut ctx.accounts.count_data;
        count_data.proposal_count += 1;

        let poll = &mut ctx.accounts.poll;
        let new_poll = Poll {
            id: count_data.proposal_count,
            title,
            options: options.clone(),
            options_count: options.len() as u8,
            bump, 
        };
        poll.set_inner(new_poll);

        Ok(())
    }

    pub fn vote_poll(ctx: Context<VotePoll>, bump: u8, voter: Pubkey, option: u8) -> Result<()> {
        let vote = &mut ctx.accounts.vote;
        let new_vote = Vote {
            poll: ctx.accounts.poll.key(),
            voter,
            option_selected: option,
            bump 
        };

        vote.set_inner(new_vote);


        Ok(())
    }
}
