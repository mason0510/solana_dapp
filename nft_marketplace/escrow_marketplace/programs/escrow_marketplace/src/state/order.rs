use anchor_lang::prelude::*;

#[account]
pub struct OrderAccount {
    pub initializer_key: Pubkey,
    pub seller_mint_token_account: Pubkey,
    pub seller_token_account: Pubkey,
    pub price: u64,
}