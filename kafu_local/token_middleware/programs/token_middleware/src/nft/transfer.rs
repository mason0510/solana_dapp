use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_option::COption;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Token, TokenAccount, Mint};
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
use crate::NftMint;

pub fn process_transfer_nft(
    ctx: Context<NftTransfer>,
) -> Result<()> {
    msg!("start transfer nft");
    token::transfer(CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token::Transfer {
            from: ctx.accounts.from_ata.to_account_info(),
            to: ctx.accounts.to_ata.to_account_info(),
            authority: ctx.accounts.from.to_account_info(),
        }), 1)?;
    ctx.accounts.to_ata.reload()?;
    token::close_account(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), token::CloseAccount {
            account: ctx.accounts.from_ata.to_account_info(),
            destination: ctx.accounts.from.to_account_info(),
            authority: ctx.accounts.from.to_account_info(),
        }),
    );
    msg!("remaining tokens: {}", ctx.accounts.to_ata.amount);
    Ok(())
}

#[derive(Accounts)]
pub struct NftTransfer<'info> {
    #[account(mut)]
    pub from_ata: Account<'info,TokenAccount>,
    /// CHECK
    #[account(mut,signer)]
    pub from: AccountInfo<'info>,
    ///CHECK: ?
    pub to: UncheckedAccount<'info>,
    #[account(
    init_if_needed,
    payer = from,
    associated_token::mint = mint,
    associated_token::authority = to,
    )]
    pub to_ata: Account<'info,TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info,System>,
    pub rent: Sysvar<'info, Rent>,
}