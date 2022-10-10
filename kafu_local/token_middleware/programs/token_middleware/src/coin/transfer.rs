use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_option::COption;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Token, TokenAccount, Mint};
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

/// coin transfer func
pub fn process_transfer_coin(
    ctx: Context<CoinTransfer>,
    amount: u64,
) -> Result<()> {
    msg!("start transfer spl token");
    token::transfer(CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token::Transfer {
            from: ctx.accounts.from_ata.to_account_info(),
            to: ctx.accounts.to_ata.to_account_info(),
            authority: ctx.accounts.from.to_account_info(),
        }), amount)?;
    ctx.accounts.to_ata.reload()?;
    msg!("remaining tokens: {}", ctx.accounts.to_ata.amount);
    Ok(())
}

/// Some accounts which transfer instruction need
#[derive(Accounts)]
pub struct CoinTransfer<'info> {
    /// resource ata account of transfer
    #[account(mut)]
    pub from_ata: Account<'info,TokenAccount>,
    /// resource  wallet of transfer
    #[account(mut)]
    pub from: Signer<'info>,
    ///CHECK: destion wallet of transfer
    pub to: AccountInfo<'info>,
    /// destion ata account of transfer
    #[account(
    init_if_needed,
    payer = from,
    associated_token::mint = coin,
    associated_token::authority = to,
    )]
    pub to_ata: Account<'info,TokenAccount>,
    /// coin mint account
    pub coin: Account<'info, Mint>,
    /// spl token program
    pub token_program: Program<'info, Token>,
    /// associated token program
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// system program
    pub system_program: Program<'info,System>,
    /// rent program
    pub rent: Sysvar<'info, Rent>,
}