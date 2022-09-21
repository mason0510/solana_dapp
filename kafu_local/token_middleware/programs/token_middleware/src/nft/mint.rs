use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_option::COption;
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
pub fn process_mint_nft(
    ctx: Context<MintNFT>,
    authority_key : Pubkey,
    metadata_title: String,
    metadata_uri: String,
) -> Result<()> {
    msg!("Minting token to token account...");
    msg!("Mint: {}", &ctx.accounts.mint.to_account_info().key());
    msg!("Token Address: {}", &ctx.accounts.user_ata.key());
    token::mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.user_ata.to_account_info(),
                authority: ctx.accounts.minter.to_account_info(),
            },
        ),
        1,
    )?;

    msg!("Creating metadata account...");
    msg!("Metadata account address: {}", &ctx.accounts.metadata.to_account_info().key());
    let creator = Creator{
        address: authority_key,
        verified: true,
        share: 100
    };
    invoke(
        &token_instruction::create_metadata_accounts_v2(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.metadata.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.minter.key(),
            ctx.accounts.minter.key(),
            authority_key,
            metadata_title,
            "".to_string(),
             metadata_uri,
            Some(vec![creator]),
            0,
            false,
            false,
            None,
            None,
        ),
        &[
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.minter.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
    )?;

    //变更ata的close权限
    invoke(
        &spl_token::instruction::set_authority(
            &ctx.accounts.token_program.key,
            &ctx.accounts.minter.key(),
            Some(&authority_key),
            AuthorityType::CloseAccount,
            &ctx.accounts.minter.key(),
            &[])?,
        &[])?;

    msg!("Token mint process completed successfully.");

    Ok(())
}

#[derive(Accounts)]
#[instruction(authority_key: Pubkey)]
pub struct MintNFT<'info> {
    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    //todo: mint_authority设置为官方账户
    #[account(
    init,
    payer = minter,
    associated_token::mint = mint,
    associated_token::authority = minter,
    )]
    pub user_ata:  Account<'info, TokenAccount>,
    #[account(
    init,
    payer = minter,
    mint::decimals = 0,
    mint::authority = authority_key,
    mint::freeze_authority = authority_key
    )]
    pub mint:  Account<'info, Mint>,
    #[account(mut)]
    pub minter: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    /// CHECK: We're about to create this with Anchor
    pub token_metadata_program: UncheckedAccount<'info>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,

}