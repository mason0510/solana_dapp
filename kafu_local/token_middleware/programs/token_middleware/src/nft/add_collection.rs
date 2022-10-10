use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_option::COption;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Token, TokenAccount,Mint};
use mpl_token_metadata::instruction;
use mpl_token_metadata::instruction::UpdateMetadataAccountArgsV2;
use mpl_token_metadata::state::{Creator, DataV2, Metadata, TokenMetadataAccount};
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


pub fn process_nft_add_collection(ctx: Context<NftAddCollection>) -> Result<()>{
    msg!("start add collection {}",ctx.accounts.metadata.key());
    let instruction = token_instruction::verify_collection(
        ctx.accounts.mpl_token_metadata.key(),
        ctx.accounts.metadata.key(),
        ctx.accounts.collection_authority.key(),
        ctx.accounts.collection_authority.key(),
        ctx.accounts.collection_mint.key(),
        ctx.accounts.collection_metadata.key(),
        ctx.accounts.collection_master_edition.key(),
            None
    );
    invoke(
        &instruction,
        &[
            ctx.accounts.collection_authority.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.collection_mint.to_account_info(),
            ctx.accounts.collection_metadata.to_account_info(),
            ctx.accounts.collection_master_edition.to_account_info(),
        ])?;
    Ok(())
}

//authority 同时是collection的mint authority也是nft的update authority
#[derive(Accounts)]
pub struct NftAddCollection<'info> {
    #[account(mut)]
    pub collection_authority: Signer<'info>,
    /// CHECK: We're about to create this with Anchor
    #[account(mut)]
    pub metadata: AccountInfo<'info>,

    /// CHECK: We're about to create this with Anchor
    pub collection_mint: UncheckedAccount<'info>,
    /// CHECK: We're about to create this with Anchor
    pub collection_metadata: UncheckedAccount<'info>,
    /// CHECK: We're about to create this with Anchor
    pub collection_master_edition: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    /// CHECK: We're about to create this with Anchor
    pub mpl_token_metadata: UncheckedAccount<'info>,
    pub token_program: Program<'info, token::Token>,
}