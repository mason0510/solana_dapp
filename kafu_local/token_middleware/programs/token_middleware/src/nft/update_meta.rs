use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_option::COption;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Token, TokenAccount,Mint};
use mpl_token_metadata::state::{Creator, Metadata, TokenMetadataAccount};
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


pub fn process_update_meta(ctx: Context<NftUpdateMeta>,new_name: Option<String>,new_uri:Option<String>) -> Result<()>{
    msg!("start process_update_meta {}",ctx.accounts.metadata.key());
    let mut data =  Metadata::from_account_info(ctx.accounts.metadata.as_ref()).unwrap().data;
    if let Some(name) = new_name{
        data.name = name;
    }
    if let Some(uri) = new_uri{
        data.uri = uri;
    }

    invoke(
        &mpl_token_metadata::instruction::update_metadata_accounts(
            ctx.accounts.mpl_token_metadata.key(),
            ctx.accounts.metadata.key(),
            ctx.accounts.authority.key(),
            None,
            Some(data),
            None,
            ),
        &[
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
        ])?;
    Ok(())
}

#[derive(Accounts)]
pub struct NftUpdateMeta<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK: We're about to create this with Anchor
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: We're about to create this with Anchor
    pub mpl_token_metadata: UncheckedAccount<'info>
}