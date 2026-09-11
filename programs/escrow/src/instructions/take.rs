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
    pub maker: SystemAccount<'info>,

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
    pub taker_ata_b: Box<Account<'info, TokenAccount>>, // 👈 Boxed to save stack space

    #[account(
        mut,
        constraint = maker_ata_b.mint == escrow_state.mint_b
    )]
    pub maker_ata_b: Box<Account<'info, TokenAccount>>, // 👈 Boxed to save stack space

    #[account(
        mut,
        constraint = taker_ata_a.mint == escrow_state.mint_a
    )]
    pub taker_ata_a: Box<Account<'info, TokenAccount>>, // 👈 Boxed to save stack space

    #[account(
        mut,
        close = maker,
        seeds = [b"escrow", maker.key().as_ref(), &escrow_state.seed.to_le_bytes()],
        bump
    )]
    pub escrow_state: Box<Account<'info, EscrowState>>, // 👈 Boxed (This is the largest custom account)

    #[account(
        mut,
        seeds = [b"vault", escrow_state.key().as_ref()],
        bump,
        token::mint = escrow_state.mint_a,
        token::authority = escrow_state
    )]
    pub escrow_vault: Box<Account<'info, TokenAccount>>, // 👈 Boxed to save stack space

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

pub fn handle_take(ctx: Context<Take>) -> Result<()> {
    // 1. Extract variables needed for the transfer
    let amount = ctx.accounts.escrow_state.amount_b;
    let escrow_vault = &ctx.accounts.escrow_vault;
    let taker_ata_a = &ctx.accounts.taker_ata_a;
    let token_program = &ctx.accounts.token_program;

    // --- CPI 1: Transfer from taker_ata_b to maker_ata_b ---
    let cpi_accounts = TransferChecked {
        from: ctx.accounts.taker_ata_b.to_account_info(),
        to: ctx.accounts.maker_ata_b.to_account_info(),
        authority: ctx.accounts.taker.to_account_info(),
        mint: ctx.accounts.mint_b.to_account_info(),
    };

    // FIX: Pass .to_account_info() instead of .key()
    let cpi_ctx = CpiContext::new(token_program.key(), cpi_accounts);
    transfer_checked(cpi_ctx, amount, ctx.accounts.mint_b.decimals)?; // Use dynamic decimals safely
    
    // --- CPI 2: Transfer from escrow_vault to taker_ata_a ---
    let vault_balance = escrow_vault.amount;
    
    let cpi_accounts_vault = TransferChecked {
        from: escrow_vault.to_account_info(),
        to: taker_ata_a.to_account_info(),
        authority: ctx.accounts.escrow_state.to_account_info(),
        mint: ctx.accounts.mint_a.to_account_info()
    };

    // FIX: Extract primitive values to variables so their references outlive the array definition
    let seed_bytes = ctx.accounts.escrow_state.seed.to_le_bytes();
    let maker_key = ctx.accounts.escrow_state.maker.key();
    let bump_bytes = [ctx.bumps.escrow_state];

    // FIX: Reconstructed seeds cleanly so the rust compiler can trace them safely
    let seeds = [
        b"escrow".as_ref(),
        maker_key.as_ref(),
        &seed_bytes,
        &bump_bytes,
    ];
    let vault_signer_seeds = &[seeds.as_ref()];

    // FIX: Pass .to_account_info() instead of .key()
    let cpi_ctx_vault = CpiContext::new_with_signer(
        token_program.key(), 
        cpi_accounts_vault, 
        vault_signer_seeds
    );

    transfer_checked(cpi_ctx_vault, vault_balance, ctx.accounts.mint_a.decimals)?;

    Ok(())
}
