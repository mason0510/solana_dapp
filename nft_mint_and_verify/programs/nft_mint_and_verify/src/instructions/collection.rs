use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_spl::{associated_token, token};
use anchor_spl::token::{Mint};
use mpl_token_metadata::{
    instruction,
};
use crate::{SetAndVerifyCollection, transfer};


pub fn set_and_verify_collection(ctx: Context<SetAndVerifyCollection>) -> Result<()> {
    let ix = instruction::set_and_verify_collection(
        mpl_token_metadata::ID,
        ctx.accounts.metadata_account.key(),
        ctx.accounts.collection_authority.key(),
        ctx.accounts.payer.key(),
        ctx.accounts.update_authority.key(),
        ctx.accounts.collection_mint.key(),
        ctx.accounts.collection_metadata.key(),
        ctx.accounts.collection_master_edition.key(),
        None,
    );
    solana_program::program::invoke_signed(
        &ix,
        &[
                ctx.accounts.metadata_account.to_account_info(), //
                ctx.accounts.collection_authority.to_account_info(), //
                ctx.accounts.payer.to_account_info(), //
                ctx.accounts.update_authority.to_account_info(), //
                ctx.accounts.collection_mint.to_account_info(), //
                ctx.accounts.collection_metadata.to_account_info(),
                ctx.accounts.collection_master_edition.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
            ctx.accounts.spl_token_metadata.to_account_info()

        ],
        &[],
    ).unwrap();


    transfer(ctx)


    /*msg!("Creating user token account...");
    msg!("User Token Address: {}", &ctx.accounts.receiver_token_account.key());
    associated_token::create(
        CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(),
            associated_token::Create {
                payer: ctx.accounts.payer.to_account_info(),
                associated_token: ctx.accounts.receiver_token_account.to_account_info(),
                authority: ctx.accounts.receiver_wallet.to_account_info(),
                mint: ctx.accounts.mint_account.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.spl_token_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ),
    )?;

    msg!("Transferring NFT...");
    msg!("sender Token Address: {}", &ctx.accounts.sender_token_account.key());
    msg!("receiver Token Address: {}", &ctx.accounts.receiver_token_account.key());
    token::transfer(
        CpiContext::new(
            ctx.accounts.spl_token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.sender_token_account.to_account_info(),
                to: ctx.accounts.receiver_token_account.to_account_info(),
                authority: ctx.accounts.payer.to_account_info(),
            }
        ),
        1
    )?;
    msg!("NFT transferred successfully.");
    Ok(())*/

}