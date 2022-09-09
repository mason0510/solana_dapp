use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{Mint, SetAuthority, TokenAccount, Transfer};
use anchor_spl::token::spl_token::instruction::AuthorityType;
use crate::constants::{VAULT_SIGNER, ORDER_SPACE,VAULT_PREFIX};
use crate::state::order::{SellOrder};

#[derive(Accounts)]
#[instruction(vault_authority_key: Pubkey, price: u64)]
pub struct Sell<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut, signer)]
    pub seller: AccountInfo<'info>,
    pub nft_mint: Account<'info, Mint>,
    //fixme: add other params as seed?
    #[account(
    init,
    seeds = [
        VAULT_PREFIX.as_ref(),
        nft_mint.key().as_ref()
    ],
    bump,
    payer = seller,
    token::mint = nft_mint,
    token::authority = seller,
    )]
    pub vault_account: Account<'info, TokenAccount>,
    #[account(
    mut,
    constraint = seller_token_account.amount == 1
    )]
    pub seller_token_account: Account<'info, TokenAccount>,
    //fixme: replace with pda?
    #[account(
        init,
        payer = seller,
        space = ORDER_SPACE,
    )]
    pub escrow_account: Account<'info, SellOrder>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub system_program: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,
}

impl<'info> Sell<'info> {
    fn into_transfer_to_pda_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self
                .seller_token_account
                .to_account_info()
                .clone(),
            to: self.vault_account.to_account_info().clone(),
            authority: self.seller.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }

    fn into_set_authority_context(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            account_or_mint: self.vault_account.to_account_info().clone(),
            current_authority: self.seller.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }
}

pub fn process_sell(
    ctx: Context<Sell>,
    price: u64,
) -> Result<()> {
    ctx.accounts.escrow_account.wallet = *ctx.accounts.seller.key;
    ctx.accounts
        .escrow_account
        .nft_token_account = *ctx
        .accounts
        .seller_token_account
        .to_account_info()
        .key;

    ctx.accounts
        .escrow_account
        .mint_account = *ctx
        .accounts
        .nft_mint
        .to_account_info()
        .key;

    ctx.accounts.escrow_account.price = price;

    //在owner里面直接指定了
    let (vault_authority, _vault_authority_bump) =
        Pubkey::find_program_address(&[
            VAULT_SIGNER,
            ctx.accounts.nft_mint.to_account_info().key.as_ref()
        ], ctx.program_id);
    token::set_authority(
        ctx.accounts.into_set_authority_context(),
        AuthorityType::AccountOwner,
        Some(vault_authority),
    )?;

    token::transfer(
        ctx.accounts.into_transfer_to_pda_context(),
        1,
    )?;

    Ok(())
}