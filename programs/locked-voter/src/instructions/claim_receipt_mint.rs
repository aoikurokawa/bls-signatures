use anchor_spl::{
    associated_token::AssociatedToken,
    token::{TokenAccount, Token}
};
use crate::*;

#[derive(Accounts)]
pub struct ClaimReceiptMint<'info> {
    #[account(mut)]
    pub stake_entry: Account<'info, StakeEntry>,

    pub original_mint: Account<'info, Mint>,
    #[account(mut)]
    pub receipt_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = stake_entry_receipt_mint_token_account.amount > 0
            && stake_entry_receipt_mint_token_account.owner == stake_entry.key() 
    )]
    pub stake_entry_receipt_mint_token_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = receipt_mint,
        associated_token::authority = payer
    )]
    pub user_receipt_mint_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>
}