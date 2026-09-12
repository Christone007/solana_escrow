pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("CmR8LJjNnojbNyfcAw3aVurgjQVY9zaBaES7UV95PKQ3");

#[program]
pub mod escrow {

use super::*;

    pub fn make(ctx: Context<Make>, amount_a: u64, amount_b: u64, seed: u64) -> Result<()> {
        make::handle_make(ctx, amount_a, amount_b, seed)
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        take::handle_take(ctx)
    }

    pub fn update(ctx: Context<Update>, new_amount: u64) -> Result<()> {
        update::handle_update(ctx, new_amount)
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        refund::handle_refund(ctx)
    }

    
}
