use crate::*;

#[derive(Accounts)]
pub struct NewLock<'info> {
    pub base: Signer<'info>,
    #[account(
        init,
        seeds = [
            b"my_khe_lock".as_ref(),
        ],
        bump,
        payer = payer,
        space = 8
    )]
    pub lock: Account<'info, Lock>,

    /// Mint of the token that can be used to join the [Lock]
    pub token_mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NewEscrow<'info> {
    pub lock: Account<'info, Lock>,

    #[account(
        init,
        seeds = [
            b"my_khe_escrow".as_ref(),
            lock.key().to_bytes().as_ref(),
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
