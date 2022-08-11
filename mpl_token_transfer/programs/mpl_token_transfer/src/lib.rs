use anchor_lang::prelude::*;

pub mod transfer;

use transfer::*;


declare_id!("BGzwb76jQtDP9hpho7WDSzFSYSJUbbZq4Jkpggb4aiuA");


#[program]
pub mod kingwo_nft {
    use super::*;

    pub fn transfer(
        ctx: Context<Transfer>,
    ) -> Result<()> {
        transfer::transfer(ctx)
    }
}