use anchor_lang::solana_program::program::invoke_signed;
use {
    anchor_lang::{
        prelude::*,
        system_program,
    },
    anchor_spl::{
        associated_token,
        token,
    },
};
use super::Transfer;



pub fn create_token_account2(ctx: &Context<Transfer>) -> Result<()>{
    msg!("Creating receiver token account..._0004");
 /*   associated_token::create(
        CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(),
            associated_token::Create {
                payer: ctx.accounts.sender_authority.to_account_info(),
                associated_token: ctx.accounts.receiver_token_account.to_account_info(),
                authority: ctx.accounts.sender_authority.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ),
    )?;*/

    invoke_signed(
        &spl_associated_token_account::create_associated_token_account(
            &ctx.accounts.sender_authority.key,
            &ctx.accounts.sender_authority.key,
            &ctx.accounts.mint.key()
        ),
        &[
            ctx.accounts.sender_authority.to_account_info(),
            ctx.accounts.receiver_token_account.to_account_info(),
            ctx.accounts.sender_authority.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
            ctx.accounts.associated_token_program.to_account_info(),
        ],
        &[]
    )?;

    /*
    {
  "info": {
    "account": "9hUYW9s2c98GfjZb6JvW62BYEt3ryxGmeMBkhgSqmZtW",
    "mint": "7YYNXbfwd5i5scpez18fTkEh2MRHJXoMHPffnWNcpFYf",
    "rentSysvar": "SysvarRent111111111111111111111111111111111",
    "source": "677NzkzkDKT9wXDMXGPUvbFp1T7XzJtZZxcRaBAaSvNa",
    "systemProgram": "11111111111111111111111111111111",
    "tokenProgram": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
    "wallet": "677NzkzkDKT9wXDMXGPUvbFp1T7XzJtZZxcRaBAaSvNa"
  },
  "type": "create"
}
    ***/

    Ok(())
}


pub fn transfer_nft(
    ctx: Context<Transfer>,
) -> Result<()> {
    msg!("Transferring NFT...");
    msg!("sender Token Address: {}", &ctx.accounts.sender_token_account.key());
    msg!("receiver Token Address: {}", &ctx.accounts.receiver_token_account.key());
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.sender_token_account.to_account_info(),
                to: ctx.accounts.receiver_token_account.to_account_info(),
                authority: ctx.accounts.sender_authority.to_account_info(),
            }
        ),
        1
    )?;
    msg!("NFT transferred successfully.");
    
    msg!("Sale completed successfully!");

    Ok(())
}