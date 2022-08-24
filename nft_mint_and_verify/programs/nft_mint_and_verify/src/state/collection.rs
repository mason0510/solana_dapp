use anchor_lang::prelude::*;
use anchor_spl::{associated_token, token};
use anchor_spl::token::{Mint, Token};
use mpl_token_metadata::state::Metadata;

#[derive(Accounts)]
pub struct SetAndVerifyCollection<'info> {
    /// CHECK:?
    #[account(mut)]
    pub metadata_account: AccountInfo<'info>,
    /// CHECK:?
    pub collection_authority: AccountInfo<'info>,
    /// CHECK:?
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK:?
    #[account(mut)]
    pub update_authority: Signer<'info>,
    /// CHECK:?
    pub collection_mint: Account<'info, Mint>,
    /// CHECK:?
    pub collection_metadata: AccountInfo<'info>,
    /// CHECK:?
    pub collection_master_edition: AccountInfo<'info>,
    /// CHECK:?
    pub rent: AccountInfo<'info>,
    // pub rent: Sysvar<'info, Rent>,
    /// CHECK:?
    pub system_program: AccountInfo<'info>,
    // pub system_program: Program<'info, System>,
    /// CHECK:?
    pub spl_token_metadata: AccountInfo<'info>,
    /// CHECK:?
    pub spl_token_program: AccountInfo<'info>,

    //for transfer
    /// CHECK:?
    #[account(mut)]
    pub mint_account: AccountInfo<'info>,
    /// CHECK:?
    #[account(mut)]
    pub sender_token_account: AccountInfo<'info>,
    /// CHECK:?
    #[account(mut)]
    pub receiver_token_account:UncheckedAccount<'info>,
    /// CHECK: We're about to create this with Anchor
    #[account(mut)]
    pub receiver_wallet: AccountInfo<'info>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,

}