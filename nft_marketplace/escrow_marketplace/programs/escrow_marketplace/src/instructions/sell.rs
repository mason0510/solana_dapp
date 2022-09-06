use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{Mint, SetAuthority, TokenAccount, Transfer};
use anchor_spl::token::spl_token::instruction::AuthorityType;
use crate::constants::ESCROW_PDA_SEED;
use crate::state::order::OrderAccount;

#[derive(Accounts)]
#[instruction(vault_account_bump: u8, initializer_amount: u64)]
pub struct Sell<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut, signer)]
    pub initializer: AccountInfo<'info>,
    pub nft_mint: Account<'info, Mint>,
    #[account(
    init,
    seeds = [b"token-seed10".as_ref()],
    bump,
    payer = initializer,
    token::mint = nft_mint,
    token::authority = initializer,
    )]
    pub vault_account: Account<'info, TokenAccount>,
    #[account(
    mut,
    constraint = seller_token_account.amount == 1
    )]
    pub seller_token_account: Account<'info, TokenAccount>,
    #[account(zero)]
    pub escrow_account: Box<Account<'info, OrderAccount>>,
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
            authority: self.initializer.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }

    fn into_set_authority_context(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            account_or_mint: self.vault_account.to_account_info().clone(),
            current_authority: self.initializer.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }
}

pub fn sell(
    ctx: Context<Sell>,
    price: u64,
) -> Result<()> {
    ctx.accounts.escrow_account.initializer_key = *ctx.accounts.initializer.key;
    ctx.accounts
        .escrow_account
        .seller_token_account = *ctx
        .accounts
        .seller_token_account
        .to_account_info()
        .key;

    ctx.accounts
        .escrow_account
        .seller_mint_token_account = *ctx
        .accounts
        .nft_mint
        .to_account_info()
        .key;

    ctx.accounts.escrow_account.price = price;

    let (vault_authority, _vault_authority_bump) =
        Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);
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