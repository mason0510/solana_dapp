pub mod state;
pub mod instructions;
pub mod errors;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token;
use anchor_spl::token::{MintTo, Token};
use instructions::*;
use state::*;

use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v2};

declare_id!("9HiRJw3dYo2MV9B1WrqFfoNjWRPS19mjVDCPqAxuMPfb");

#[program]
pub mod nft_mint_and_verify {
    use anchor_spl::token::{initialize_account, initialize_mint};
    use crate::state::MintNFT;
    use crate::instructions;
    use super::*;

    pub fn mint_to(ctx: Context<MintNFT>, title: String, uri: String, symbol: String) -> Result<()> {
        mint_nft(ctx,title,symbol,uri)?;
        Ok(())
    }

    pub fn account_init(ctx: Context<AccountInit>) -> Result<()> {
        instructions::account_init(ctx)?;
        Ok(())
    }

    pub fn collection_add(ctx: Context<SetAndVerifyCollection>) -> Result<()> {
        instructions::set_and_verify_collection(ctx)?;
        Ok(())
    }
}

