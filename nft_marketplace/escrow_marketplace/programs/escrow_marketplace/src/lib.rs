pub mod constants;
pub mod instructions;
pub mod errors;
pub mod state;

use anchor_lang::prelude::*;
use anchor_spl::associated_token;
use anchor_spl::token::{self, CloseAccount, Mint, SetAuthority, TokenAccount, Transfer};
use spl_token::instruction::AuthorityType;
use std::str::FromStr;

//fixme: 必须在这里导出来才能在program里面用
use crate::instructions::cancel::*;
use crate::instructions::buy::*;
use crate::instructions::sell::*;

declare_id!("Hv49DNdC6CUSwK3MWH5gj5BfLUU64r2ANXwdhaaRceGD");

#[program]
pub mod escrow_marketplace {


    use super::*;

    pub fn sell_external(ctx: Context<Sell>,price: u64) -> Result<()> {
        sell(ctx,price)
    }

    pub fn cancel_external(ctx: Context<Cancel>) -> Result<()> {
        cancel(ctx)
    }

    pub fn buy_external(ctx: Context<Buy>) -> Result<()> {
        buy(ctx)
    }
}





