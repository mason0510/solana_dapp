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
pub fn process_mint_coin(
    ctx: Context<CoinMint>,
    metadata_title: String,
    metadata_symbol: String,
    metadata_uri: String,
    init_supply:u64,
) -> Result<()> {

    system_program::create_account(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            system_program::CreateAccount {
                from: ctx.accounts.minter.to_account_info(),
                to: ctx.accounts.mint.to_account_info(),
            },
        ),
        1461600,
        82,
        &ctx.accounts.token_program.key(),
    )?;

    msg!("Initializing mint account...");
    msg!("Mint: {}", &ctx.accounts.mint.key());
    token::initialize_mint(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::InitializeMint {
                mint: ctx.accounts.mint.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ),
        9,
        &ctx.accounts.minter.key(),
        Some(&ctx.accounts.minter.key()),
    )?;

    associated_token::create(
        CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(),
            associated_token::Create {
                payer: ctx.accounts.minter.to_account_info(),
                associated_token: ctx.accounts.user_ata.to_account_info(),
                authority: ctx.accounts.minter.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ),
    )?;


    //=========
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
        init_supply,
    )?;

    msg!("Creating metadata account...");
    msg!("Metadata account address: {}", &ctx.accounts.metadata.to_account_info().key());
    invoke(
        &token_instruction::create_metadata_accounts_v2(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.metadata.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.minter.key(),
            ctx.accounts.minter.key(),
            ctx.accounts.authority.key(),
            metadata_title,
            metadata_symbol,
            metadata_uri,
            None,
            0,
            false,
            true,
            None,
            None,
        ),
        &[
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.minter.to_account_info(),
            ctx.accounts.authority.to_account_info(),
        ],
    )?;
    msg!("Token mint process completed successfully.");

    Ok(())
}

//当前mint key和ata不能在anchor里面同时创建，待进一步调查，先按照原来逻辑
#[derive(Accounts)]
//#[instruction(authority_key: Pubkey)]
pub struct CoinMint<'info> {
    /// CHECK: We're about to create this with Metaplex
    pub authority: UncheckedAccount<'info>,
    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /* #[account(
     init,
     payer = minter,
     associated_token::mint = mint,
     associated_token::authority = minter,
     )]*/
    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub user_ata:  UncheckedAccount<'info>,
    /*    #[account(
        init,
        payer = minter,
        mint::decimals = 0,
        mint::authority = minter,
        mint::freeze_authority = minter
        )]*/
    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub mint:  Signer<'info>,
    /*    /// CHECK: We're about to create this with Anchor
        #[account(
       mut
        )]
        pub user_ata:  UncheckedAccount<'info>,
        /// CHECK: We're about to create this with Anchor
        #[account(
     mut
        )]
        pub mint:  UncheckedAccount<'info>,*/


    #[account(mut)]
    pub minter: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    /// CHECK: We're about to create this with Anchor
    pub token_metadata_program: UncheckedAccount<'info>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,

}