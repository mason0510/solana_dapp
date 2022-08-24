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
        ID as TOKEN_METADATA_ID,
        instruction as token_instruction,
    },
};

use crate::state::MintNFT;

pub fn collection_add(ctx: Context<MintNFT>,title: String, uri: String, symbol: String) -> Result<()>{
    Ok(())
}