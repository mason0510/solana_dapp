pub mod constants;
pub mod errors;
pub mod state;
pub mod buy;
pub mod sell;
pub mod cancel;

use anchor_lang::prelude::*;
use anchor_spl::associated_token;
use anchor_spl::token::{self, CloseAccount, Mint, SetAuthority, TokenAccount, Transfer};
use spl_token::instruction::AuthorityType;
use std::str::FromStr;
use crate::sell::*;
use crate::buy::*;
use crate::cancel::*;

declare_id!("Hv49DNdC6CUSwK3MWH5gj5BfLUU64r2ANXwdhaaRceGD");

#[program]
pub mod escrow_marketplace {

    use super::*;

    pub fn sell(ctx: Context<Sell>, _vault_authority_key: Pubkey, price: u64) -> Result<()> {
        process_sell(ctx,price)
    }

    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
        process_cancel(ctx)
    }

    pub fn buy(ctx: Context<Buy>) -> Result<()> {
        process_buy(ctx)
    }
}





