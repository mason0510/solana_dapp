use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{CloseAccount, TokenAccount, Transfer};
use crate::constants::{VAULT_SIGNER,MARKET_SETTING};
use crate::state::order::SellOrder;

#[derive(Accounts)]
pub struct Cancel<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut, signer)]
    pub seller: AccountInfo<'info>,
    #[account(mut)]
    pub vault_account: Account<'info, TokenAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub vault_authority: AccountInfo<'info>,
    #[account(mut)]
    pub seller_token_account: Account<'info, TokenAccount>,
    #[account(
    mut,
    constraint = escrow_account.wallet == *seller.key,
    constraint = escrow_account.nft_token_account == *seller_token_account.to_account_info().key,
    close = seller
    )]
    pub escrow_account: Box<Account<'info, SellOrder>>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,
}

impl<'info> Cancel<'info> {
    fn into_transfer_to_initializer_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.vault_account.to_account_info().clone(),
            to: self
                .seller_token_account
                .to_account_info()
                .clone(),
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

pub fn process_cancel(ctx: Context<Cancel>) -> Result<()> {
    let mint_account_seed = ctx.accounts.escrow_account.mint_account.key().as_ref().to_owned();

    let (_vault_authority, vault_authority_bump) =
        Pubkey::find_program_address(&[
            VAULT_SIGNER,
            mint_account_seed.as_slice()
        ], ctx.program_id);
    let authority_seeds = &[&VAULT_SIGNER[..],mint_account_seed.as_slice(),&[vault_authority_bump]];

    token::transfer(
        ctx.accounts
            .into_transfer_to_initializer_context()
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