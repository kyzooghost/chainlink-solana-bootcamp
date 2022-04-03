use anchor_lang::prelude::*;

declare_id!("6Lj72Xn6hTYNQ8JTz8uVgrmDYBkVyjQy8xXMLSKN8Wo3");

// Define new Anchor program
#[program]
pub mod gm_anchor {
   use super::*;
   pub fn execute(ctx: Context<Execute>) -> ProgramResult {
       // Get account from context
       let gm_account = &mut ctx.accounts.gm_account;

       // Store name string into specified account
       gm_account.number = gm_account.number + 1;
       msg!("You've said GM {} times", gm_account.number);
       Ok(())
   }
}

#[derive(Accounts)]
pub struct Execute<'info> {
    // Define account passed into the execute instruction
   #[account(init, payer = user, space = 8 + 4)]
   pub gm_account: Account<'info, GreetingAccount>,
   #[account(mut)]
   pub user: Signer<'info>,
   pub system_program: Program<'info, System>,
}

#[account]
pub struct GreetingAccount {
   pub number: u32,
}