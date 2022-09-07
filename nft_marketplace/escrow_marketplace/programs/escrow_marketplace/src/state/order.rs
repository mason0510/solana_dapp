use anchor_lang::{prelude::*,AnchorSerialize,AnchorDeserialize};

#[account]
#[derive(Debug)]
pub struct OrderAccount {
    pub seller: Pubkey,
    pub seller_mint_token_account: Pubkey,
    pub seller_token_account: Pubkey,
    pub price: u64,
}