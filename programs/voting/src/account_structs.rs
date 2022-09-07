use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [b"govern".as_ref()],
        bump,
        payer = payer,
        space = 8 + PollCount::LEN,
    )]
    pub count_data: Account<'info, PollCount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreatePoll<'info> {
    #[account(
        mut,
        seeds = [b"govern".as_ref()],
        bump,
    )]
    pub count_data: Account<'info, PollCount>,
}

#[derive(Accounts)]
pub struct VotePoll {}
