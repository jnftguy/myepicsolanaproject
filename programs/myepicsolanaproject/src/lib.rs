use anchor_lang::prelude::*;

declare_id!("Cpph6pLhunrNrZ7eYtJwDheW86YedVihr2RfqRoqiVcW");

#[program]
pub mod myepicsolanaproject {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
    Ok(())
  }
}

#[derive(Accounts)]
pub struct StartStuffOff {}


#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
}