use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, TransferChecked, transfer_checked};

use crate::EscrowState;



#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info> {
// initialize the escrow: Creates the escrow_state and the vault that holds creators token_a
// hence, the vault must be a Token account of mint_a
    #[account(mut)]
    pub maker: Signer<'info>,

    pub mint_a: Account<'info, Mint>,
    pub mint_b: Account<'info, Mint>,

    #[account(
        mut,
        constraint = maker_ata_a.mint == mint_a.key(),
        owner = maker.key()
    )]
    pub maker_ata_a: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = maker,
        space = EscrowState::DISCRIMINATOR.len() + EscrowState::INIT_SPACE,
        seeds = [b"escrow", maker.key().as_ref(), &seed.to_le_bytes()],
        bump
    )]
    pub escrow_state: Account<'info, EscrowState>,

    #[account(
        init,
        payer = maker,
        seeds = [b"vault", escrow_state.key().as_ref()],
        bump,
        token::mint = mint_a,
        token::authority = escrow_state
    )]
    pub escrow_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

pub fn handle_make(ctx:Context<Make>, amount_a: u64, amount_b: u64, seed: u64) -> Result<()> {
    // transfer amount_a of token_a from maker_ata_a to escrow_vault and store data in escrow_state

    let maker = &ctx.accounts.maker;
    let mint_a = &ctx.accounts.mint_a;
    let mint_b = &ctx.accounts.mint_b;
    let escrow_vault = &ctx.accounts.escrow_vault;

    let maker_ata_a = &ctx.accounts.maker_ata_a;

    // Update the Escrow State
    ctx.accounts.escrow_state.maker = maker.key();
    ctx.accounts.escrow_state.mint_a = mint_a.key();
    ctx.accounts.escrow_state.mint_b = mint_b.key();
    ctx.accounts.escrow_state.amount_a = amount_a;
    ctx.accounts.escrow_state.amount_b = amount_b;
    ctx.accounts.escrow_state.seed = seed;



    // MAKE TRANSFER from maker's ata_a to vault

    let cpi_accounts = TransferChecked {
        from: maker_ata_a.to_account_info(),
        to: escrow_vault.to_account_info(),
        authority: maker.to_account_info(),
        mint: mint_a.to_account_info()
    };

    let cpi_program = ctx.accounts.token_program.key();

    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    transfer_checked(cpi_ctx, amount_a, 6)?;

    Ok(())
}
   