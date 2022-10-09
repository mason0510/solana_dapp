#![deny(missing_docs)]
//#![deny(warnings)]

use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_option::COption;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Token, TokenAccount,Mint};
use mpl_token_metadata::state::Creator;
use spl_token::instruction::AuthorityType;
use {
    anchor_lang::{
        prelude::*,
        solana_program::program::invoke,
        system_program,
    },
    anchor_spl::{
        associated_token,
        token,
    },
    mpl_token_metadata::{
        instruction as token_instruction
    },
};
use crate::errors::MiddlewareError;

/// increase coin supply
pub fn process_issue_coin(
    ctx: Context<CoinIssue>,
    amount:u64,
) -> Result<()> {
    msg!("start issue Mint: {}", &ctx.accounts.mint.to_account_info().key());
    msg!("User ATA Address: {}", &ctx.accounts.user_ata.key());
    token::mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.user_ata.to_account_info(),
                authority: ctx.accounts.minter.to_account_info(),
            },
        ),
        amount,
    )?;
    Ok(())
}

/// Coin issue struct
#[derive(Accounts)]
pub struct CoinIssue<'info> {
    /// CHECK:
    #[account(mut)]
    pub user_ata:  AccountInfo<'info>,
    /// coin mint accout
    #[account(constraint = mint.mint_authority == COption::Some(minter.key()) @MiddlewareError::PermissionDenied)]
    pub mint:  Account<'info,Mint>,
    /// coin creator
    #[account(mut)]
    pub minter: Signer<'info>,
    /// spl token program address
    pub token_program: Program<'info, token::Token>,

}