use anchor_lang::prelude::*;
use anchor_spl::{associated_token, token};
use anchor_spl::token::{CloseAccount, Mint, TokenAccount, Transfer};
use crate::constants::*;
use crate::errors::MarketError;
use std::str::FromStr;
use crate::state::order::OrderAccount;

#[derive(Accounts)]
pub struct Buy<'info> {
    #[account(signer,mut)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub buyer: AccountInfo<'info>,
    #[account(mut)]
    pub buyer_coin_account: Box<Account<'info, TokenAccount>>,  // K coin
    #[account(
    constraint = *k_coin_mint_account.to_account_info().key == Pubkey::from_str(K_COIN).unwrap() @ MarketError::NotSupportCoin,
    )]
    pub k_coin_mint_account: Box<Account<'info, Mint>>,  // K coin
    pub nft_token_mint_account: Box<Account<'info, Mint>>,  // nft mint account
    #[account(
    init_if_needed,
    payer = buyer,
    associated_token::mint = nft_token_mint_account,
    associated_token::authority = buyer,
    )]
    pub buyer_token_account: Account<'info, TokenAccount>, //nft token
    #[account(
    init_if_needed,
    payer = buyer,
    associated_token::mint = k_coin_mint_account,
    associated_token::authority = seller,
    )]
    pub seller_coin_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub seller_token_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub seller: AccountInfo<'info>,
    #[account(
    mut,
    constraint = escrow_account.price <= buyer_coin_account.amount                                           @ MarketError::InSufficientFunds,
    constraint = escrow_account.seller_token_account == *seller_token_account.to_account_info().key     ,
    constraint = escrow_account.seller_mint_token_account == *nft_token_mint_account.to_account_info().key   @ MarketError::NftNotMatched,
    constraint = escrow_account.seller == *seller.key                                               @ MarketError::SellerNotMatched,
    close = seller
    )]
    pub escrow_account: Box<Account<'info, OrderAccount>>,
    #[account(mut)]
    pub vault_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub vault_authority: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,
    //sys account
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> Buy<'info> {
    fn into_transfer_to_initializer_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.buyer_coin_account.to_account_info().clone(),
            to: self
                .seller_coin_account
                .to_account_info()
                .clone(),
            authority: self.buyer.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
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

pub fn process_buy(ctx: Context<Buy>) -> Result<()> {
    let (_vault_authority, vault_authority_bump) =
        Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);
    let authority_seeds = &[&ESCROW_PDA_SEED[..], &[vault_authority_bump]];
    //send K coin to seller from buyer
    token::transfer(
        ctx.accounts.into_transfer_to_initializer_context(),
        ctx.accounts.escrow_account.price,
    )?;

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