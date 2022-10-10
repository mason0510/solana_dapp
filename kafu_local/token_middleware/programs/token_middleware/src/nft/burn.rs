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


pub fn process_nft_burn(ctx: Context<NftBurn>) -> Result<()>{
    msg!("start freeze {}",ctx.accounts.mint.key());
    invoke(
        &spl_token::instruction::burn(
            &ctx.accounts.token_program.key,
            &ctx.accounts.user_ata.key(),
            &ctx.accounts.mint.key(),
            &ctx.accounts.authority.key(),
            &[&ctx.accounts.authority.key()],
            1)?,
        &[
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.user_ata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            //ctx.accounts.mint_owner.to_account_info(),
        ])?;
    Ok(())
}

#[derive(Accounts)]
pub struct NftBurn<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK: We're about to create this with Anchor
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    /// CHECK: We're about to create this with Anchor
    #[account(mut)]
    pub user_ata: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
}