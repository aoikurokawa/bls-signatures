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
#[instruction(bump: u8, title: String, desctiption_link: String)]
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
            + 4 + title.as_bytes().len() 
            + 4 + desctiption_link.as_bytes().len() 
    )]
    pub poll: Account<'info, Poll>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(bump: u8, voter: Pubkey)]
pub struct VotePoll<'info> {
    pub poll: Account<'info, Poll>,
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
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
