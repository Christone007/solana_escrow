use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, TransferChecked, transfer_checked};
use anchor_lang::system_program::System;

use crate::state::EscrowState;


#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(
        address = escrow_state.maker
    )]
    pub maker: UncheckedAccount<'info>,

    #[account(
        address = escrow_state.mint_a
    )]
    pub mint_a: Account<'info, Mint>,

    #[account(
        address = escrow_state.mint_b
    )]
    pub mint_b: Account<'info, Mint>,

    #[account(
        mut,
        constraint = taker_ata_b.mint == escrow_state.mint_b
    )]
    pub taker_ata_b: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = maker_ata_b.mint == escrow_state.mint_b
    )]
    pub maker_ata_b: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = taker_ata_a.mint == escrow_state.mint_a
    )]
    pub taker_ata_a: Account<'info, TokenAccount>,

    #[account(
        mut,
        close = maker,
        seeds = [b"escrow", maker.key().as_ref(), &escrow_state.seed.to_le_bytes()],
        bump
    )]
    pub escrow_state: Account<'info, EscrowState>,

    #[account(
        mut,
        seeds = [b"vault", escrow_state.key().as_ref()],
        bump,
        token::mint = escrow_state.mint_a,
        token::authority = escrow_state
    )]
    pub escrow_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

pub fn handle_take(ctx:Context<Take>) -> Result<()> {
    // transfer from taker_ata_b to maker_ata_b

    let amount = ctx.accounts.escrow_state.amount_b;
    let escrow_state = &ctx.accounts.escrow_state;
    let escrow_vault = &ctx.accounts.escrow_vault;
    let taker_ata_a = &ctx.accounts.taker_ata_a;

    let cpi_accounts = TransferChecked {
        from: ctx.accounts.taker_ata_b.to_account_info(),
        to: ctx.accounts.maker_ata_b.to_account_info(),
        authority: ctx.accounts.taker.to_account_info(),
        mint: ctx.accounts.mint_b.to_account_info(),
    };

    let token_program = &ctx.accounts.token_program;

    let cpi_ctx = CpiContext::new(token_program.key(), cpi_accounts);

    transfer_checked(cpi_ctx, amount, 6)?;
    
    // transfer from vault to taker_ata_a
    let vault_balance = escrow_vault.amount;
    
    let cpi_accounts_vault = TransferChecked {
        from: escrow_vault.to_account_info(),
        to: taker_ata_a.to_account_info(),
        authority: escrow_state.to_account_info(),
        mint: ctx.accounts.mint_a.to_account_info()
    };



    let vault_signer_seeds: &[&[&[u8]]] = &[&[
        b"escrow",
        ctx.accounts.escrow_state.maker.as_ref(),
        &ctx.accounts.escrow_state.seed.to_le_bytes(),
        &[ctx.bumps.escrow_state],
    ]];

    // let signer = &[&vault_signer_seeds[..]];

    let cpi_ctx_vault = CpiContext::new_with_signer(token_program.key(), cpi_accounts_vault, vault_signer_seeds);

    transfer_checked(cpi_ctx_vault, vault_balance, 6)?;

    // the constraints should now close both escrow_state and escrow_vault

    Ok(())
}