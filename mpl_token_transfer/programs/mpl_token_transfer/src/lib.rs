use anchor_lang::prelude::*;

pub mod transfer;

use transfer::*;

use anchor_spl::{associated_token, token};

declare_id!("9st7EdZS5GgDjjKVFk7gR62Smp1YtirhdNCoQax7mxJP");


#[program]
pub mod kingwo_nft {

    use super::*;


    pub fn transfer(
        ctx: Context<Transfer>,
    ) -> Result<()> {
        if ctx.accounts.receiver_token_account.owner == ctx.accounts.token_program.key{
            msg!("ATA {} already exists",ctx.accounts.receiver_token_account.key);
        }else {
            create_token_account(&ctx)?;
        }
        transfer::transfer_nft(ctx)?;
        Ok(())
    }


}


/***
    #[account(mut)]
    pub mint: Account<'info, token::Mint>,
    #[account(mut)]
    pub owner_token_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub owner_authority: Signer<'info>,
    /// CHECK: We're about to create this with Anchor
    #[account(mut)]
    pub buyer_token_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub buyer_authority: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
*/

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut)]
    pub mint: Account<'info, token::Mint>,
    #[account(mut)]
    pub sender_token_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub sender_authority: Signer<'info>,
    //#[account(init, payer = user, space = 8 + 8)]
    //#[account(mut)]
    //pub to_token_account: UncheckedAccount<'info>,
    /// CHECK: We're about to create this with Anchor
    /*#[account(
    init,
    payer = sender_authority,
    associated_token::mint = mint,
    associated_token::authority = sender_authority,
    )]*/
    //pub receiver_token_account:Account<'info,token::TokenAccount>,
    #[account(mut)]
    pub receiver_token_account:UncheckedAccount<'info>,
    /// CHECK: We're about to create this with Anchor
    #[account(mut)]
    pub receiver_wallet: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}