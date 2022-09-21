pub mod coin;
pub mod nft;

use anchor_lang::prelude::*;
use nft::mint::*;


declare_id!("8ZjekeVj2PHuVmaTX2Ti7vv1tZy3THJ9fZY2JJxwMaQv");

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

    pub fn nft_mint(ctx: Context<MintNFT>, authority_key : Pubkey,
                    name: String,
                    uri: String,) -> Result<()>{
        process_mint_nft(ctx,authority_key,name,uri)
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