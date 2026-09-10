use anchor_lang::prelude::*;

use crate::state::EscrowState;

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mut,
        seeds = [b"escrow", maker.key().as_ref(), &escrow_state.seed.to_le_bytes()],
        bump
    )]
    pub escrow_state: Account<'info, EscrowState>
}

pub fn handle_update(ctx: Context<Update>, new_amount: u64) -> Result<()> {
    // change the amount of token b requested
    ctx.accounts.escrow_state.amount_b = new_amount;
    Ok(())
}