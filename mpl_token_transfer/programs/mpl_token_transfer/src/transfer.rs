use std::str::FromStr;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_spl::token::TokenAccount;
use spl_associated_token_account::get_associated_token_address;
use {
    anchor_lang::{
        prelude::*,
        system_program,
    },
    anchor_spl::{
        associated_token,
        token,
    },
};
use super::Transfer;



pub fn create_token_account(ctx: &Context<Transfer>) -> Result<()>{
    msg!("Creating receiver token account..._0004");
    //fixme: 其实_receiver_token_account = ctx.accounts.receiver_token_account
    /*   let receiver_token_account =
           get_associated_token_address(ctx.accounts.receiver_wallet.key, ctx.accounts.token_program.key);
   */
    associated_token::create(
        CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(),
            associated_token::Create {
                payer: ctx.accounts.sender_authority.to_account_info(),
                associated_token: ctx.accounts.receiver_token_account.to_account_info(),
                authority: ctx.accounts.receiver_wallet.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ),
    )?;

    Ok(())
}

pub fn transfer_nft(
    ctx: Context<Transfer>,
) -> Result<()> {
    msg!("Transferring NFT...");
    msg!("sender Token Address: {}", &ctx.accounts.sender_token_account.key());
    msg!("receiver Token Address: {}", &ctx.accounts.receiver_token_account.key());
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.sender_token_account.to_account_info(),
                to: ctx.accounts.receiver_token_account.to_account_info(),
                authority: ctx.accounts.sender_authority.to_account_info(),
            }
        ),
        1
    )?;
    msg!("NFT transferred successfully.");
    
    msg!("Sale completed successfully!");

    Ok(())
}