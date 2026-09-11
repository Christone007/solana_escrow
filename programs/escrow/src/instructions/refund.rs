use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, TransferChecked, Mint, transfer_checked};

use crate::state::EscrowState;

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mut,
        close = maker,
        seeds = [b"escrow", maker.key().as_ref(), &escrow_state.seed.to_le_bytes()],
        bump
    )]
    pub escrow_state: Account<'info, EscrowState>,

    #[account(
        mut,
        close = maker,
        seeds = [b"vault", escrow_state.key().as_ref()],
        bump,
        token::mint = escrow_state.mint_a,
        token::authority = escrow_state
    )]
    pub escrow_vault: Account<'info, TokenAccount>,

    #[account(
        address = escrow_state.mint_a
    )]
    pub mint_a: Account<'info, Mint>,

     #[account(
        mut,
        constraint = maker_ata_a.mint == escrow_state.mint_a.key(),
        owner = maker.key()
    )]
    pub maker_ata_a: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>
}


pub fn handle_refund(ctx: Context<Refund>) -> Result<()> {
    // transfer everything from vault to maker_ata_a
    let vault = &ctx.accounts.escrow_vault;
    let escrow = &ctx.accounts.escrow_state;
    let maker_ata_a = &ctx.accounts.maker_ata_a;
    let mint_a = &ctx.accounts.mint_a;
    let token_program = &ctx.accounts.token_program;
    let vault_balance = vault.amount;

    let cpi_accounts = TransferChecked {
        from: vault.to_account_info(),
        to: maker_ata_a.to_account_info(),
        mint: mint_a.to_account_info(),
        authority: escrow.to_account_info()
    };

    let vault_signer_seeds: &[&[&[u8]]] = &[&[
        b"escrow",
        ctx.accounts.escrow_state.maker.as_ref(),
        &ctx.accounts.escrow_state.seed.to_le_bytes(),
        &[ctx.bumps.escrow_state],
    ]];

    // let signer = &[&vault_signer_seeds[..]];

    let cpi_ctx_vault = CpiContext::new_with_signer(token_program.key(), cpi_accounts, vault_signer_seeds);

    transfer_checked(cpi_ctx_vault, vault_balance, 6)?;

    // the constraint closes the vault and escrow automatically
    Ok(())
}