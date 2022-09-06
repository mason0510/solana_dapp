use anchor_lang::prelude::*;
use anchor_spl::associated_token;
use anchor_spl::token::{self, CloseAccount, Mint, SetAuthority, TokenAccount, Transfer};
use spl_token::instruction::AuthorityType;
use std::str::FromStr;

declare_id!("Hv49DNdC6CUSwK3MWH5gj5BfLUU64r2ANXwdhaaRceGD");
const K_COIN:  &'static str = "5d1i4wKHhGXXkdZB22iKD1SqU6pkBeTCwFEMqo7xy39h";

#[error_code]
pub enum MarketError {
    #[msg("InSufficientFunds")]
    InSufficientFunds,
    #[msg("NotSupportCoin")]
    NotSupportCoin,
    #[msg("NftNotMatched")]
    NftNotMatched,
    #[msg("SellerNotMatched")]
    SellerNotMatched
}

#[program]
pub mod escrow_marketplace {
    use std::str::FromStr;
    use super::*;

    const ESCROW_PDA_SEED: &[u8] = b"escrow";

    pub fn initialize(
        ctx: Context<Initialize>,
        _vault_account_bump: u8, //todo: 暂时无用
        price: u64,
    ) -> Result<()> {
        ctx.accounts.escrow_account.initializer_key = *ctx.accounts.initializer.key;
        ctx.accounts
            .escrow_account
            .seller_token_account = *ctx
            .accounts
            .seller_token_account
            .to_account_info()
            .key;

        ctx.accounts
            .escrow_account
            .seller_mint_token_account = *ctx
            .accounts
            .nft_mint
            .to_account_info()
            .key;

        ctx.accounts.escrow_account.price = price;

        let (vault_authority, _vault_authority_bump) =
            Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);
        token::set_authority(
            ctx.accounts.into_set_authority_context(),
            AuthorityType::AccountOwner,
            Some(vault_authority),
        )?;

        token::transfer(
            ctx.accounts.into_transfer_to_pda_context(),
            1,
        )?;

        Ok(())
    }

    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
        let (_vault_authority, vault_authority_bump) =
            Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);
        let authority_seeds = &[&ESCROW_PDA_SEED[..], &[vault_authority_bump]];

        token::transfer(
            ctx.accounts
                .into_transfer_to_initializer_context()
                .with_signer(&[&authority_seeds[..]]),
            1,
        )?;

        token::close_account(
            ctx.accounts
                .into_close_context()
                .with_signer(&[&authority_seeds[..]]),
        )?;

        Ok(())
    }

    pub fn exchange(ctx: Context<Exchange>) -> Result<()> {
        let (_vault_authority, vault_authority_bump) =
            Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);
        let authority_seeds = &[&ESCROW_PDA_SEED[..], &[vault_authority_bump]];

        //send K coin to seller from buyer
        token::transfer(
            ctx.accounts.into_transfer_to_initializer_context(),
            ctx.accounts.escrow_account.price,
        )?;

        //send nft to buyer from vault_account
        token::transfer(
            ctx.accounts
                .into_transfer_to_taker_context()
                .with_signer(&[&authority_seeds[..]]),
            1,
        )?;

        token::close_account(
            ctx.accounts
                .into_close_context()
                .with_signer(&[&authority_seeds[..]]),
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(vault_account_bump: u8, initializer_amount: u64)]
pub struct Initialize<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut, signer)]
    pub initializer: AccountInfo<'info>,
    pub nft_mint: Account<'info, Mint>,
    #[account(
        init,
        seeds = [b"token-seed10".as_ref()],
        bump,
        payer = initializer,
        token::mint = nft_mint,
        token::authority = initializer,
    )]
    pub vault_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = seller_token_account.amount == 1
    )]
    pub seller_token_account: Account<'info, TokenAccount>,
    #[account(zero)]
    pub escrow_account: Box<Account<'info, EscrowAccount>>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub system_program: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Cancel<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut, signer)]
    pub initializer: AccountInfo<'info>,
    #[account(mut)]
    pub vault_account: Account<'info, TokenAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub vault_authority: AccountInfo<'info>,
    #[account(mut)]
    pub seller_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = escrow_account.initializer_key == *initializer.key,
        constraint = escrow_account.seller_token_account == *seller_token_account.to_account_info().key,
        close = initializer
    )]
    pub escrow_account: Box<Account<'info, EscrowAccount>>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,
}


/***
 #[account(
        init_if_needed,
        payer = user_receiving,
        associated_token::mint = mint_of_token_being_sent,
        associated_token::authority = user_receiving,
    )]
*/
#[derive(Accounts)]
pub struct Exchange<'info> {
    #[account(signer,mut)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub buyer: AccountInfo<'info>,
    #[account(mut)]
    pub buyer_coin_account: Box<Account<'info, TokenAccount>>,  // K coin
    #[account(
        constraint = *k_coin_mint_account.to_account_info().key == Pubkey::from_str(K_COIN).unwrap() @ MarketError::NotSupportCoin,
    )]
    pub k_coin_mint_account: Box<Account<'info, Mint>>,  // K coin
    pub nft_token_mint_account: Box<Account<'info, Mint>>,  // nft mint account
    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = nft_token_mint_account,
        associated_token::authority = buyer,
    )]
    pub buyer_token_account: Account<'info, TokenAccount>, //nft token
    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = k_coin_mint_account,
        associated_token::authority = seller,
    )]
    pub seller_coin_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub seller_token_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub seller: AccountInfo<'info>,
    #[account(
        mut,
        constraint = escrow_account.price <= buyer_coin_account.amount                                           @ MarketError::InSufficientFunds,
        constraint = escrow_account.seller_token_account == *seller_token_account.to_account_info().key     ,
        constraint = escrow_account.seller_mint_token_account == *nft_token_mint_account.to_account_info().key   @ MarketError::NftNotMatched,
        constraint = escrow_account.initializer_key == *seller.key                                               @ MarketError::SellerNotMatched,
        close = seller
    )]
    pub escrow_account: Box<Account<'info, EscrowAccount>>,
    #[account(mut)]
    pub vault_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub vault_authority: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,
    //sys account
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,

}

#[account]
pub struct EscrowAccount {
    pub initializer_key: Pubkey,
    pub seller_mint_token_account: Pubkey,
    pub seller_token_account: Pubkey,
    pub price: u64,
}

impl<'info> Initialize<'info> {
    fn into_transfer_to_pda_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self
                .seller_token_account
                .to_account_info()
                .clone(),
            to: self.vault_account.to_account_info().clone(),
            authority: self.initializer.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }

    fn into_set_authority_context(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            account_or_mint: self.vault_account.to_account_info().clone(),
            current_authority: self.initializer.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }
}

impl<'info> Cancel<'info> {
    fn into_transfer_to_initializer_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.vault_account.to_account_info().clone(),
            to: self
                .seller_token_account
                .to_account_info()
                .clone(),
            authority: self.vault_authority.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }

    fn into_close_context(&self) -> CpiContext<'_, '_, '_, 'info, CloseAccount<'info>> {
        let cpi_accounts = CloseAccount {
            account: self.vault_account.to_account_info().clone(),
            destination: self.initializer.clone(),
            authority: self.vault_authority.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }
}

impl<'info> Exchange<'info> {
    fn into_transfer_to_initializer_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.buyer_coin_account.to_account_info().clone(),
            to: self
                .seller_coin_account
                .to_account_info()
                .clone(),
            authority: self.buyer.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }

    fn into_transfer_to_taker_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.vault_account.to_account_info().clone(),
            to: self.buyer_token_account.to_account_info().clone(),
            authority: self.vault_authority.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }

    fn into_close_context(&self) -> CpiContext<'_, '_, '_, 'info, CloseAccount<'info>> {
        let cpi_accounts = CloseAccount {
            account: self.vault_account.to_account_info().clone(),
            destination: self.seller.clone(),
            authority: self.vault_authority.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }
}
