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
}