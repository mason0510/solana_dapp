use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_spl::token::{Mint};
use mpl_token_metadata::{
    instruction,
};
use crate::SetAndVerifyCollection;


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
    /***

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
    pub system_program: AccountInfo<'info>,
    /// CHECK:?
    pub rent: AccountInfo<'info>,
    */
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
            ctx.accounts.mpl_token_metadata.to_account_info()

        ],
        &[],
    ).map_err(Into::into)
}