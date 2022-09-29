extern crate core;

use std::fs::Metadata;
use std::rc::Rc;
use anchor_client::solana_sdk::signature::read_keypair_file;
use anchor_client::solana_sdk::signature::{Keypair, Signer};
use anchor_client::{Client, Cluster};
use anyhow::Result;
use std::str::FromStr;
use anchor_client::anchor_lang::Key;
use anchor_client::anchor_lang::prelude::Pubkey;
use anchor_client::anchor_lang::solana_program::program_pack::Pack;
use borsh::BorshDeserialize;
use mpl_token_metadata::state::Collection;
use solana_client::nonce_utils::get_account;
use solana_sdk::account::ReadableAccount;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::timing::timestamp;
use spl_associated_token_account::get_associated_token_address;
use spl_token::state::{Account, AccountState};


use token_middleware::instruction as token_middleware_instructions;
use token_middleware::accounts as token_middleware_accounts;


use crate::utils::{find_master_edition_pda, find_metadata_pda};
use crate::{MEM_COLLECTION_MINT, MPL_TOKEN_METADATA_ACCOUNT, NFT_MINT_CONTRACT, SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID, SPL_PROGRAM_ID, SYSTEM_PROGRAM_ID, SYSTEM_RENT_ID, TOKEN_MIDDLEWARE};

//可以替第三方铸造
pub fn mint(client: &Client,uri:&str,name:&str,collection:Option<Collection>) -> Result<Pubkey> {
    let program = client.program(Pubkey::from_str(TOKEN_MIDDLEWARE).unwrap());
    let payer_key = program.payer();
    let minter_key = program.payer();
    let nft_mint_key = Keypair::new();
    std::thread::sleep(std::time::Duration::from_secs_f32(0.001));
    let user_ata = get_associated_token_address(&minter_key, &nft_mint_key.pubkey());
    let metadata_address = find_metadata_pda(&nft_mint_key.pubkey());
    let master_key = find_master_edition_pda(&nft_mint_key.pubkey());
    println!("nft mint key1 {}", nft_mint_key.pubkey().to_string());

    let mint_build = program
        .request()
        .accounts(token_middleware_accounts::NftMint{
            authority: Pubkey::from_str("9EZZmeAE16RsKPxbL9VXBjGFooPsKfePRxfyLJrp8umu").unwrap(),
            metadata: metadata_address,
            user_ata,
            mint: nft_mint_key.pubkey(),
            minter: minter_key,
            rent: Pubkey::from_str(SYSTEM_RENT_ID).unwrap(),
            system_program: Pubkey::from_str(SYSTEM_PROGRAM_ID).unwrap(),
            token_program: Pubkey::from_str(SPL_PROGRAM_ID).unwrap(),
            token_metadata_program: Pubkey::from_str(MPL_TOKEN_METADATA_ACCOUNT).unwrap(),
            associated_token_program: Pubkey::from_str(SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID).unwrap(),
        })
        .args(token_middleware_instructions::NftMint{
            authority_key: payer_key,
            name: name.to_string(),
            uri: uri.to_string(),
            collection: collection.clone()
        });
    if collection.is_some(){
        let add_collection_build = program
            .request()
            .accounts(token_middleware_accounts::NftAddCollection{
                collection_authority: program.payer(),
                //update_authority: Pubkey::from_str("9EZZmeAE16RsKPxbL9VXBjGFooPsKfePRxfyLJrp8umu").unwrap(),
                metadata: metadata_address,
                collection_mint: collection.as_ref().unwrap().key,
                collection_metadata: find_metadata_pda(&collection.as_ref().unwrap().key),
                collection_master_edition: find_master_edition_pda(&collection.as_ref().unwrap().key),
                system_program: Pubkey::from_str(SYSTEM_PROGRAM_ID).unwrap(),
                mpl_token_metadata: Pubkey::from_str(MPL_TOKEN_METADATA_ACCOUNT).unwrap(),
                token_program: Pubkey::from_str(SPL_PROGRAM_ID).unwrap(),
            })
            .args(token_middleware_instructions::NftAddCollection);

        let to = Pubkey::from_str("9EZZmeAE16RsKPxbL9VXBjGFooPsKfePRxfyLJrp8umu").unwrap();
        let transfer_build = program
            .request()
            .accounts(token_middleware_accounts::NftTransfer{
                from_ata: user_ata,
                from: program.payer(),
                to,
                to_ata: get_associated_token_address(&to,&nft_mint_key.pubkey()),
                mint: nft_mint_key.pubkey(),
                token_program: Pubkey::from_str(SPL_PROGRAM_ID).unwrap(),
                associated_token_program: Pubkey::from_str(SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID).unwrap(),
                system_program: Pubkey::from_str(SYSTEM_PROGRAM_ID).unwrap(),
                rent:Pubkey::from_str(SYSTEM_RENT_ID).unwrap(),
            })
            .args(token_middleware_instructions::NftTransfer);

        let mint_res = program
            .request()
            .instruction(mint_build.instructions()?.first().unwrap().to_owned())
            .instruction(add_collection_build.instructions()?.first().unwrap().to_owned())
            .instruction(transfer_build.instructions()?.first().unwrap().to_owned())
            .signer(&nft_mint_key)
            .send()?;
        println!("call res {}", mint_res);
    }else {
        let mint_res = program
            .request()
            .instruction(mint_build.instructions()?.first().unwrap().to_owned())
            .signer(&nft_mint_key)
            .send()?;
        println!("call res {}", mint_res);
    }

    println!("nft mint key2 {}", nft_mint_key.pubkey().to_string());

    Ok(nft_mint_key.pubkey())
}




pub fn mint_master_edition(client: &Client,uri:&str,name:&str) -> Result<Pubkey> {
    let program = client.program(Pubkey::from_str(TOKEN_MIDDLEWARE).unwrap());
    let nft_mint_key = Keypair::new();
    println!("nft mint key {}", nft_mint_key.pubkey().to_string());

    let user_ata = get_associated_token_address(&program.payer(), &nft_mint_key.pubkey());
    let metadata_address = find_metadata_pda(&nft_mint_key.pubkey());

    let now = format!("timestamp_{}",timestamp() % 100000 );
    let mint_build = program
        .request()
        .accounts(token_middleware_accounts::NftMintMasterEdition{
            authority: program.payer(),
            metadata: metadata_address,
            user_ata,
            mint: nft_mint_key.pubkey(),
            minter: program.payer(),
            master_edition: find_master_edition_pda(&nft_mint_key.pubkey()),
            rent: Pubkey::from_str(SYSTEM_RENT_ID).unwrap(),
            system_program: Pubkey::from_str(SYSTEM_PROGRAM_ID).unwrap(),
            token_program: Pubkey::from_str(SPL_PROGRAM_ID).unwrap(),
            token_metadata_program: Pubkey::from_str(MPL_TOKEN_METADATA_ACCOUNT).unwrap(),
            associated_token_program: Pubkey::from_str(SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID).unwrap(),
        })
        .args(token_middleware_instructions::NftMintMasterEdition{
            collection: None,
            name:name.to_string(),
            uri:uri.to_string(),
        });

    let mint_res = program
        .request()
        .instruction(mint_build.instructions()?.first().unwrap().to_owned())
        .signer(&nft_mint_key)
        .send()?;
    println!("call res {}", mint_res);
    println!("collection mint key {}", nft_mint_key.pubkey().to_string());

    Ok(nft_mint_key.pubkey())
}


pub fn transfer() -> Result<()>{
    let to= Pubkey::from_str("6iytHt6hJ9szSvNVL713JoioXPLfoPGjKKTSCUhUtH73").unwrap();
    let mint = Pubkey::from_str("9ge2nYxd3N7veVf47JdvF6EF8QmpmnUTAK8FxdQvTeQe").unwrap();
    let client = crate::get_wallet("~/.config/solana/id.json".to_string());
    let program = client.program(Pubkey::from_str(TOKEN_MIDDLEWARE).unwrap());
    let from_ata = get_associated_token_address(&program.payer(), &mint);
    let to_ata = get_associated_token_address(&to, &mint);


    let mint_build = program
        .request()
        .accounts(token_middleware_accounts::NftTransfer{
            from_ata,
            from: program.payer(),
            to,
            to_ata,
            mint: mint,
            token_program: Pubkey::from_str(SPL_PROGRAM_ID).unwrap(),
            associated_token_program: Pubkey::from_str(SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID).unwrap(),
            system_program: Pubkey::from_str(SYSTEM_PROGRAM_ID).unwrap(),
            rent:Pubkey::from_str(SYSTEM_RENT_ID).unwrap(),
        })
        .args(token_middleware_instructions::NftTransfer);

    let mint_res = program
        .request()
        .instruction(mint_build.instructions()?.first().unwrap().to_owned())
        .send()?;
    println!("call res {}", mint_res);
    Ok(())
}