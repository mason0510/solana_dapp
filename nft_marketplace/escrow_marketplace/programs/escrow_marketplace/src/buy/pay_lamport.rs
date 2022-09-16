use std::borrow::Borrow;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use anchor_lang::prelude::*;
use anchor_spl::{associated_token, token};
use anchor_spl::token::{CloseAccount, Mint, TokenAccount, Transfer};
use crate::constants::*;
use crate::errors::MarketError;
use std::str::FromStr;
use anchor_lang::system_program;
use crate::state::order::{SellOrder, Settings};
use crate::constants::{MARKET_SETTING};


#[derive(Accounts)]
pub struct PayLamport<'info> {
    #[account(
        signer,
        mut,
        constraint = *Rc::try_unwrap(buyer.lamports.clone()).unwrap().into_inner()  >= escrow_account.price   @MarketError::InSufficientFunds
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub buyer: AccountInfo<'info>,
    //fixme：确认下是否需要考虑手续费
    #[account(mut)]
    pub nft_token_mint_account: Box<Account<'info, Mint>>,  // nft mint account
    #[account(
    init_if_needed,
    payer = buyer,
    associated_token::mint = nft_token_mint_account,
    associated_token::authority = buyer,
    )]
    pub buyer_token_account: Account<'info, TokenAccount>, //nft token
    #[account(mut)]
    pub seller_token_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub seller: AccountInfo<'info>,
    #[account(
    mut,
    constraint = escrow_account.nft_token_account == *seller_token_account.to_account_info().key,
    constraint = escrow_account.mint_account == *nft_token_mint_account.to_account_info().key                @ MarketError::NftNotMatched,
    constraint = escrow_account.wallet == *seller.key                                                        @ MarketError::SellerNotMatched,
    close = seller
    )]
    pub escrow_account: Box<Account<'info, SellOrder>>,
    #[account(mut)]
    pub vault_account: Box<Account<'info, TokenAccount>>,
    #[account(seeds = [MARKET_SETTING.as_ref()],bump)]
    pub setting_account: Box<Account<'info, Settings>>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub vault_authority: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,
    //sys account
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> PayLamport<'info> {
    fn into_transfer_to_initializer_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, system_program::Transfer<'info>> {
        let test1 = *Rc::try_unwrap(self.buyer.lamports.clone()).unwrap().into_inner();
        CpiContext::new(
            self.system_program.to_account_info(),
            system_program::Transfer {
                from: self.buyer.to_account_info(),
                to: self.seller.to_account_info(),
            }
        )
    }

    fn into_transfer_to_taker_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.vault_account.to_account_info().clone(),
            to: self.buyer_token_account.to_account_info().clone(),
            authority: self.vault_authority.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }

    fn into_close_context(&self) -> CpiContext<'_, '_, '_, 'info, CloseAccount<'info>> {
        let cpi_accounts = CloseAccount {
            account: self.vault_account.to_account_info().clone(),
            destination: self.seller.clone(),
            authority: self.vault_authority.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }
}

pub fn process_pay_lamport(ctx: Context<PayLamport>) -> Result<()> {
    if ctx.accounts.escrow_account.receive_coin.is_some(){
        return Err(MarketError::NotSupportCoin.into());
    }

    //todo: sub market fee
    system_program::transfer(
        ctx.accounts.into_transfer_to_initializer_context(),
        ctx.accounts.escrow_account.price
    )?;

    let mint_account_seed = ctx.accounts.escrow_account.mint_account.key().as_ref().to_owned();
    let (_vault_authority, vault_authority_bump) =
        Pubkey::find_program_address(&[VAULT_SIGNER,mint_account_seed.as_slice()], ctx.program_id);
    let authority_seeds = &[&VAULT_SIGNER[..], mint_account_seed.as_slice(),&[vault_authority_bump]];
    //send K coin to seller from buyer


    //send nft to buyer from vault_account
    token::transfer(
        ctx.accounts
            .into_transfer_to_taker_context()
            .with_signer(&[&authority_seeds[..]]),
        1,
    )?;

    token::close_account(
        ctx.accounts
            .into_close_context()
            .with_signer(&[&authority_seeds[..]]),
    )?;

    Ok(())
}