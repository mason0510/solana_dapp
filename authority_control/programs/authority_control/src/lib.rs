// #region code
use anchor_lang::prelude::*;

declare_id!("D5AxD1SpagtoL9oCdofxx7unpbvy3Zps8CFmE4F1Rrod");

#[program]
pub mod authority_control {
    use super::*;

    #[state]
    pub struct Counter {
        pub authority: Pubkey,
        pub count: u64,
    }

    impl Counter {
        pub fn new(ctx: Context<Auth>) -> anchor_lang::Result<Self> {
            Ok(Self {
                authority: *ctx.accounts.authority.key,
                count: 0,
            })
        }

        pub fn increment(&mut self, ctx: Context<Auth>) -> anchor_lang::Result<()> {
            if &self.authority != ctx.accounts.authority.key {
                return Err(error!(ErrorCode::Unauthorized));
            }
            self.count += 1;
            Ok(())
        }
    }
}

#[derive(Accounts)]
pub struct Auth<'info> {
    authority: Signer<'info>,
}
// #endregion code

#[error_code]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
}
