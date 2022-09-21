extern crate core;

use anchor_client::solana_sdk::signature::read_keypair_file;
use anchor_client::solana_sdk::signature::{Keypair, Signer};
use anchor_client::Client;
use anyhow::Result;
use std::str::FromStr;
use anchor_client::anchor_lang::prelude::Pubkey;
use spl_associated_token_account::get_associated_token_address;


use token_middleware::instruction as token_middleware_instructions;
use token_middleware::accounts as token_middleware_accounts;


use crate::utils::{find_master_edition_pda, find_metadata_pda};
use crate::{MEM_COLLECTION_MINT, MPL_TOKEN_METADATA_ACCOUNT, NFT_MINT_CONTRACT, SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID, SPL_PROGRAM_ID, SYSTEM_PROGRAM_ID, SYSTEM_RENT_ID, TOKEN_MIDDLEWARE};

//可以替第三方铸造
pub fn mint_nft(client: &Client) -> Result<Pubkey> {
    //FiTpF8mATTwcvLcSusp3fYXm6GQfisL5umnUZJ962zxx
    let wallet3 = read_keypair_file(&*shellexpand::tilde(
        "/Users/eddy/work/repo/solana/solana_dapp/my_wallet/3.json",
    ))
    .expect("Example requires a keypair file");

    let program = client.program(Pubkey::from_str(TOKEN_MIDDLEWARE).unwrap());
    let to_wallet = program.payer();
    let payer_key = program.payer();

    let nft_mint_key = Keypair::new();
    println!("nft mint key {}", nft_mint_key.pubkey().to_string());

    //当前记忆碎皮的集合的meta_account,权限已经给了付鸿
    //let memorise_mint_account = "6P64iPbit6iUbwMj55pXXEu7GxUaE9jPVqWCmomyqPph";

    let user_ata = get_associated_token_address(&to_wallet, &nft_mint_key.pubkey());
    let metadata_address = find_metadata_pda(&nft_mint_key.pubkey());
    let master_key = find_master_edition_pda(&nft_mint_key.pubkey());

    println!("{},{},{},{},{},{}",metadata_address,user_ata,nft_mint_key.pubkey(),wallet3.pubkey(),payer_key,master_key);
    let mint_build = program
        .request()
        .accounts(token_middleware_accounts::MintNFT{
            metadata: metadata_address,
            user_ata,
            mint: nft_mint_key.pubkey(),
            minter: wallet3.pubkey(),
            rent: Pubkey::from_str(SYSTEM_RENT_ID).unwrap(),
            system_program: Pubkey::from_str(SYSTEM_PROGRAM_ID).unwrap(),
            token_program: Pubkey::from_str(SPL_PROGRAM_ID).unwrap(),
            token_metadata_program: Pubkey::from_str(MPL_TOKEN_METADATA_ACCOUNT).unwrap(),
            associated_token_program: Pubkey::from_str(SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID).unwrap(),
        })
        .args(token_middleware_instructions::NftMint{
            authority_key: payer_key,
            name: "test1".to_string(),
            uri: "https://bafybeiagelxwxuundel3rjqydvunf24llrg4e2a5l4fje27arsdzhdgaqu.ipfs.nftstorage.link/0.json".to_string(),
        });

    let mint_res = program
        .request()
        .instruction(mint_build.instructions()?.first().unwrap().to_owned())
        .signer(&nft_mint_key)
        .signer(&wallet3)
        .send()?;
    println!("call res {}", mint_res);
    println!("nft mint key {}", nft_mint_key.pubkey().to_string());

    Ok(nft_mint_key.pubkey())
}

fn transfer(){
    todo!()
    /*
        let receiver_token_account = get_associated_token_address(&wallet3.pubkey(), &nft_mint_key.pubkey());

    let transfer_instruction = spl_token::instruction::transfer(
        &Pubkey::from_str(SPL_PROGRAM_ID).unwrap(),
        &receiver_token_account,
        &nft_token_account,
        &wallet3.pubkey(),
        &[&wallet3.pubkey()],
        1,
    )
    .unwrap();
    let _transfer_res = program
        .request()
        .instruction(transfer_instruction)
        .signer(&wallet3)
        .send()
        .unwrap();*/
}