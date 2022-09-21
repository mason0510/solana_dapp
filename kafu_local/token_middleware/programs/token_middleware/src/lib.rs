pub mod coin;
pub mod nft;

use anchor_lang::prelude::*;

/***
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub puppet: Account<'info, Data>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetData<'info> {
    #[account(mut)]
    pub puppet: Account<'info, Data>,
}

#[account]
pub struct Data {
    pub data: u64,
}

**/

declare_id!("D8yTyPU9tSvJc8EuaUqRcvYsAj6SuPoYFg1uZG6istQB");

#[program]
pub mod token_middleware {
    use super::*;

    pub fn coin_mint(ctx: Context<SetData>) -> Result<()>{
        todo!()
    }

    pub fn coin_issue(ctx: Context<SetData>) -> Result<()>{
        todo!()
    }

    pub fn coin_freeze(ctx: Context<SetData>) -> Result<()>{
        todo!()
    }

    pub fn nft_rename(ctx: Context<SetData>) -> Result<()>{
        todo!()
    }

    pub fn nft_burn(ctx: Context<SetData>) -> Result<()>{
        todo!()
    }
    pub fn nft_freeze(ctx: Context<SetData>) -> Result<()>{
        todo!()
    }
    //need collection mint authority
    pub fn nft_add_collection(ctx: Context<SetData>) -> Result<()>{
        todo!()
    }

    pub fn nft_mint(ctx: Context<SetData>) -> Result<()>{
        todo!()
    }
}

#[derive(Accounts)]
pub struct SetData<'info> {
    #[account(mut)]
    pub puppet: Account<'info, Data>,
}

#[account]
pub struct Data {
    pub data: u64,
}