pub mod coin;
pub mod nft;

use anchor_lang::prelude::*;
use nft::mint::*;
use nft::mint_master_edition::*;
use nft::freeze::*;
use nft::add_collection::*;
use mpl_token_metadata::state::Collection;
use coin::mint::*;
use coin::issue::*;
use coin::transfer::*;
use nft::transfer::*;
use nft::burn::*;
use nft::update_meta::*;

declare_id!("8ZjekeVj2PHuVmaTX2Ti7vv1tZy3THJ9fZY2JJxwMaQv");

#[program]
pub mod token_middleware {
    use super::*;

    //coin interface for user
    pub fn coin_transfer(ctx: Context<CoinTransfer>, amount: u64) -> Result<()> {
        process_transfer_coin(ctx, amount)
    }

    //coin interface for manager
    pub fn coin_mint(ctx: Context<CoinMint>, name: String, symbol: String, uri: String, init_supply: u64) -> Result<()> {
        process_mint_coin(ctx, name, symbol, uri, init_supply)
    }

    pub fn coin_issue(ctx: Context<CoinIssue>, amount: u64) -> Result<()> {
        process_issue_coin(ctx, amount)
    }

    pub fn coin_freeze(ctx: Context<SetData>) -> Result<()> {
        todo!()
    }

    pub fn coin_thaw(ctx: Context<SetData>) -> Result<()> {
        todo!()
    }

    //nft interface for user
    pub fn nft_transfer(ctx: Context<NftTransfer>) -> Result<()> {
        process_transfer_nft(ctx)
    }

    pub fn nft_mint(ctx: Context<NftMint>, authority_key: Pubkey,
                    name: String,
                    uri: String,
                    collection: Option<Collection>
    ) -> Result<()> {
        process_mint_nft(ctx, authority_key, name, uri,collection)
    }

    //for public nft
    pub fn nft_mint_master_edition(ctx: Context<NftMintMasterEdition>, collection: Option<Collection>,
                                   name: String,
                                   uri: String, ) -> Result<()> {
        process_mint_nft_master_edition(ctx, collection, name, uri)
    }

    //nft interface for manager
    pub fn nft_burn(ctx: Context<NftBurn>) -> Result<()> {
        process_nft_burn(ctx)
    }

    pub fn nft_add_collection(ctx: Context<NftAddCollection>) -> Result<()> {
        process_nft_add_collection(ctx)
    }

    pub fn nft_freeze(ctx: Context<NftFreeze>) -> Result<()> {
        process_freeze(ctx)
    }

    pub fn nft_thaw(ctx: Context<SetData>) -> Result<()> {
        todo!()
    }

    pub fn nft_update_meta(ctx: Context<NftUpdateMeta>,name:Option<String>,uri:Option<String>) -> Result<()> {
        process_update_meta(ctx, name, uri)
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