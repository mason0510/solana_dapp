use anchor_lang::{prelude::*,AnchorSerialize,AnchorDeserialize};

#[derive(AnchorSerialize,AnchorDeserialize,Debug, Clone)]
pub enum SupportCoin {
    KCoin(Pubkey),
}

#[account]
#[derive(Debug)]
pub struct SellOrder {
    //wallet pubkey
    pub wallet: Pubkey,
    //nft mint account
    pub mint_account: Pubkey,
    // nft token account of seller on this nft
    pub nft_token_account: Pubkey,
    pub price: u64,
}