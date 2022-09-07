use anchor_lang::prelude::*;
use crate::state::*;


#[derive(Accounts)] 
pub struct CreatePoll<'info> {
    #[account(
        init, 
        payer = payer,
        space = 8 + 8,
    )]
    pub count_data: Account<'info, PollCount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
     
}

#[derive(Accounts)] 
pub struct VotePoll{

}


#[derive(Accounts)]
pub struct Initialize {}
