pub mod coin;
pub mod nft;

use anchor_lang::prelude::*;
use nft::mint::*;
use nft::mint_master_edition::*;
use nft::freeze::*;
use nft::add_collection::*;


declare_id!("8ZjekeVj2PHuVmaTX2Ti7vv1tZy3THJ9fZY2JJxwMaQv");

#[program]
pub mod token_middleware {
    //use crate::nft::freeze::{Freeze, process_freeze};
    use super::*;

    //coin interface for user
    pub fn coin_mint(ctx: Context<SetData>) -> Result<()>{ todo!() }
    pub fn coin_transfer(ctx: Context<SetData>) -> Result<()>{
        todo!()
    }

    //coin interface for manager
    pub fn coin_issue(ctx: Context<SetData>) -> Result<()>{
        todo!()
    }
    pub fn coin_freeze(ctx: Context<SetData>) -> Result<()>{
        todo!()
    }
    pub fn coin_thaw(ctx: Context<SetData>) -> Result<()>{
        todo!()
    }

    //nft interface for user
    pub fn nft_transfer(ctx: Context<SetData>) -> Result<()>{
        todo!()
    }
    pub fn nft_mint(ctx: Context<NftMint>, authority_key : Pubkey,
                    name: String,
                    uri: String,) -> Result<()>{
        process_mint_nft(ctx,authority_key,name,uri)
    }

    pub fn nft_mint_master_edition(ctx: Context<NftMintMasterEdition>, authority_key : Pubkey,
                    name: String,
                    uri: String,) -> Result<()>{
        process_mint_nft_master_edition(ctx,authority_key,name,uri)
    }

    //nft interface for manager
    pub fn nft_burn(ctx: Context<SetData>) -> Result<()>{
        todo!()
    }
    pub fn nft_add_collection(ctx: Context<NftAddCollection>) -> Result<()>{
        process_nft_add_collection(ctx)
    }
    pub fn nft_freeze(ctx: Context<NftFreeze>) -> Result<()>{
        process_freeze(ctx)
    }
    pub fn nft_thaw(ctx: Context<SetData>) -> Result<()>{
        todo!()
    }
    pub fn nft_rename(ctx: Context<SetData>) -> Result<()>{
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