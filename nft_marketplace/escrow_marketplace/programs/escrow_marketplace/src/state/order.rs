use anchor_lang::{prelude::*,AnchorSerialize,AnchorDeserialize};

#[account]
#[derive(Debug)]
pub struct SellOrder {
    //wallet pubkey
    pub wallet: Pubkey,
    //nft mint account
    pub mint_account: Pubkey,
    // nft token account of seller on this nft
    pub nft_token_account: Pubkey,
    //buyer should pay coin,default solana or spl-token
    pub receive_coin: Option<Pubkey>,
    pub price: u64,
}

//space 32+5*32+2+8=202
//默认支持sol
#[account]
#[derive(Debug,Default)]
pub struct Settings {
    //market owner
    pub authority: Pubkey,
    //reserve field
    //max support 5 coins
    pub support_coins: Vec<Pubkey>,
    pub fee_ratio: u16,
}