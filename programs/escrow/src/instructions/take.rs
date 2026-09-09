#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    taker: Signer<'info>,

    #[account(
        address = escrow_state.maker
    )]
    maker: AccountInfo<'info>,

    #[account(
        mut,
        constraint = taker_ata_b.mint == escrow_state.mint_b
    )]
    taker_ata_b: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = maker_ata_b.mint == escrow_state.mint_b
    )]
    maker_ata_b: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = taker_ata_a.mint == escrow_state.mint_a
    )]
    taker_ata_a: Account<'info, TokenAccount>,

    #[account(
        mut,
        close = maker,
        seeds = [b"escrow", maker.key().as_ref(), &escrow_state.seed.to_le_bytes()],
        bump
    )]
    escrow_state: Account<'info, EscrowState>,

    #[account(
        mut,
        close = taker,
        seeds = [b"vault", escrow_state.key().as_ref()],
        bump,
        token::mint = mint_a,
        token::authority = escrow_state
    )]
    escrow_vault: Account<'info, TokenAccount>,

    token_program: Program<'info, Token>,
    system_program: Program<'info, System>
}

pub fn handle_take(ctx:Context<Take>) -> Result<()> {
    
}
// transfer from taker_ata_b to maker_ata_b
// transfer from vault to taker_ata_a
// close escrow_state
// close escrow_vault by sending SOL balance to taker