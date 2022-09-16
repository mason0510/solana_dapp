pub mod constants;
pub mod errors;
pub mod state;
pub mod buy;
pub mod sell;
pub mod cancel;
pub mod init_settings;
pub mod update_settings;

use anchor_lang::prelude::*;
use anchor_spl::associated_token;
use anchor_spl::token::{self, CloseAccount, Mint, SetAuthority, TokenAccount, Transfer};
use spl_token::instruction::AuthorityType;
use std::str::FromStr;

use crate::sell::*;
use crate::cancel::*;
use crate::update_settings::*;
use crate::init_settings::*;
use crate::buy::pay_spl_token::*;
use crate::buy::pay_lamport::*;

declare_id!("D8yTyPU9tSvJc8EuaUqRcvYsAj6SuPoYFg1uZG6istQB");

#[program]
pub mod escrow_marketplace {
    use crate::buy::pay_lamport::process_pay_lamport;
    use crate::buy::pay_spl_token::process_pay_spl_token;
    use crate::buy::pay_lamport::PayLamport;
    use crate::buy::pay_spl_token::PaySplToken;
    use super::*;

    pub fn sell(ctx: Context<Sell>, _vault_authority_key: Pubkey, receive_coin: Option<Pubkey>, price: u64) -> Result<()> {
        process_sell(ctx,receive_coin,price)
    }

    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
        process_cancel(ctx)
    }

    pub fn buy_and_pay_lamport(ctx: Context<PayLamport>) -> Result<()> {
        process_pay_lamport(ctx)
    }
    pub fn buy_and_pay_spl_token(ctx: Context<PaySplToken>) -> Result<()> {
        process_pay_spl_token(ctx)
    }

    pub fn init_settings(ctx: Context<InitSettings>,support_coins: Vec<Pubkey>,fee_ratio: u16) -> Result<()>{
        process_init_settings(ctx,support_coins,fee_ratio)
    }
    pub fn update_settings(ctx: Context<UpdateSettings>,support_coins: Vec<Pubkey>,fee_ratio: u16,new_authority: Option<Pubkey>) -> Result<()>{
        process_update_settings(ctx,support_coins,fee_ratio,new_authority)
    }
}





