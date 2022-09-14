use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [
            b"my_khe_governor".as_ref(), 
        ],
        bump,
        payer = payer,
        space = 8 + PollCount::LEN
    )]
    pub count_data: Account<'info, PollCount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct CreatePoll<'info> {
    #[account(mut)]
    pub count_data: Account<'info, PollCount>,
    #[account(
        init,
        seeds = [
            b"my_khe_proposal".as_ref(),
            count_data.proposal_count.to_le_bytes().as_ref()
        ],
        bump,
        payer = payer, 
        space = Poll::LEN 
    )]
    pub poll: Account<'info, Poll>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActivatePoll<'info> {
    /// The [Poll]
    #[account(mut)]
    pub poll: Account<'info, Poll>,
}

#[derive(Accounts)]
pub struct CancelPoll<'info> {
    #[account(mut)]
    pub poll: Account<'info, Poll>,
}

#[derive(Accounts)]
#[instruction(_bump: u8, title: String, desctiption_link: String)]
pub struct CreatePollMeta<'info> {
    /// The [Poll]
    pub poll: Account<'info, Poll>,
    /// Proposer of the poll
    pub proposer: Signer<'info>,
    /// The [PollMeta]
    #[account(
        init,
        seeds = [
            b"my_khe_proposal_meta".as_ref(),
            poll.key().as_ref()
        ],
        bump,
        payer = payer,
        space = PollMeta::LEN
            + 4 + title.as_bytes().len() 
            + 4 + desctiption_link.as_bytes().len() 
    )]
    pub poll_meta: Account<'info, PollMeta>,
    /// Payer of the [PollMeta]
    #[account(mut)]
    pub payer: Signer<'info>,
    /// System Program
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(bump: u8, voter: Pubkey)]
pub struct VotePoll<'info> {
    /// Poll being voted on
    pub poll: Account<'info, Poll>,

    /// The vote
    #[account(
        init, 
        seeds = [
            b"my_khe_vote".as_ref(),
            poll.key().as_ref(),
            voter.as_ref()
        ],
        bump,
        payer = payer,
        space = Vote::LEN
    )]
    pub vote: Account<'info, Vote>, 

    /// Payer of the Vote
    #[account(mut)]
    pub payer: Signer<'info>,

    /// System program
    pub system_program: Program<'info, System>,
}
