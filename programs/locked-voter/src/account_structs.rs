use anchor_spl::token::{Token, TokenAccount};

use crate::*;

#[derive(Accounts)]
pub struct NewLock<'info> {
    pub base: Signer<'info>,
    #[account(
        init,
        seeds = [
            b"my_khe_locker".as_ref(),
        ],
        bump,
        payer = payer,
        space = 8
    )]
    pub locker: Account<'info, Locker>,

    /// Mint of the token that can be used to join the [Lock]
    pub token_mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NewEscrow<'info> {
    pub locker: Account<'info, Locker>,

    #[account(
        init,
        seeds = [
            b"my_khe_escrow".as_ref(),
            locker.key().to_bytes().as_ref(),
        ],
        bump,
        payer = payer,
        space = 8
    )]
    pub escrow: Account<'info, Escrow>,

    /// CHECK: Authority of the [Escrow] to be created
    pub escrow_owner: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Lock<'info> {
    /// [Locker]
    #[account(mut)]
    pub locker: Account<'info, Locker>,

    /// [Escrow]
    #[account(mut, has_one = locker)]
    pub escrow: Account<'info, Escrow>,

    /// Token account held by the [Escrow]
    #[account(
        mut,
        constraint = escrow.tokens == escrow_tokens.key()
    )]
    pub escrow_tokens: Account<'info, TokenAccount>,

    /// Authority of the [Escrow] and [Self::source_tokens]
    pub escrow_owner: Signer<'info>,

    /// The source of deposited tokens
    #[account(mut)]
    pub source_tokens: Account<'info, TokenAccount>,

    /// Token program
    pub token_program: Program<'info, Token>,
}
