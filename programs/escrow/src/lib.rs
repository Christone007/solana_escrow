pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("HS7TLs5KSoFwnQCU6xuMtuxZNUdQeYkBUdjhiK45WkGm");

#[program]
pub mod escrow {

use super::*;

    pub fn make(ctx: Context<Make>, amount_a: u64, amount_b: u64, seed: u64) -> Result<()> {
        make::handle_make(ctx, amount_a, amount_b, seed)
    }
}
