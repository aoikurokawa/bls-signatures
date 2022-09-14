use anchor_lang::prelude::*;

mod account_structs;
mod events;
mod state;

use account_structs::*;
use events::*;
use state::*;

declare_id!("Fx2TKLRC5V8Xu6R1w42C6k7NUXr35qVudXX86jk6RVky");

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

    pub fn create_poll(ctx: Context<CreatePoll>, bump: u8) -> Result<()> {
        let count_data = &mut ctx.accounts.count_data;

        let poll = &mut ctx.accounts.poll;
        let new_poll = Poll {
            index: count_data.proposal_count,
            bump,
            proposer: ctx.accounts.payer.key(),
            for_votes: 0,
            against_votes: 0,
            created_at: Clock::get()?.unix_timestamp,
            activated_at: 0,
            canceled_at: 0,
            voting_ends_at: 0,
        };
        poll.set_inner(new_poll);

        count_data.proposal_count += 1;

        emit!(PollCreateEvent {
            poll: poll.key(),
            index: poll.index,
        });

        Ok(())
    }

    pub fn create_poll_meta(
        ctx: Context<CreatePollMeta>,
        _bump: u8,
        title: String,
        description_link: String,
    ) -> Result<()> {
        let poll_meta = &mut ctx.accounts.poll_meta;
        let new_poll_meta = PollMeta {
            poll: ctx.accounts.poll.key(),
            title,
            description_link,
        };
        poll_meta.set_inner(new_poll_meta);
        Ok(())
    }

    pub fn vote_poll(ctx: Context<VotePoll>, bump: u8, voter: Pubkey, option: u8) -> Result<()> {
        let vote = &mut ctx.accounts.vote;
        let new_vote = Vote {
            poll: ctx.accounts.poll.key(),
            voter,
            option_selected: option,
            bump,
        };

        vote.set_inner(new_vote);

        Ok(())
    }
}
