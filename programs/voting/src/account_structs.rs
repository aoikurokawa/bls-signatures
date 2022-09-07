use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Initialize<'info> {
   #[account(
        init,
        seeds = [b"initialize_voting".as_ref()],
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
   #[account(mut)]
    pub count_data: Account<'info, PollCount>,
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[derive(Accounts)] 
pub struct VotePoll{

}
