extern crate core;

use anchor_client::solana_sdk::signature::read_keypair_file;
use anchor_client::solana_sdk::signature::{Keypair, Signer};
use anchor_client::Client;
use anyhow::Result;
use std::str::FromStr;
use anchor_client::anchor_lang::prelude::Pubkey;
use spl_associated_token_account::get_associated_token_address;

#[cfg(feature = "serde-feature")]
use {
    serde::{Deserialize, Serialize},
    serde_with::{As, DisplayFromStr},
};

use nft_mint_and_verify::accounts as nft_accounts;
use nft_mint_and_verify::instruction as nft_instructions;

use crate::utils::{find_master_edition_pda, find_metadata_pda};
use crate::{
    MEM_COLLECTION_MINT, MPL_TOKEN_METADATA_ACCOUNT, NFT_MINT_CONTRACT,
    SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID, SPL_PROGRAM_ID, SYSTEM_PROGRAM_ID, SYSTEM_RENT_ID,
};

//todo: 这里先转给了3.json,再转给了自己，改为直接转自己
pub fn simple_mint(client: &Client) -> Result<Pubkey> {
    //FiTpF8mATTwcvLcSusp3fYXm6GQfisL5umnUZJ962zxx
    let wallet3 = read_keypair_file(&*shellexpand::tilde(
        "/Users/eddy/work/repo/solana/solana_dapp/my_wallet/3.json",
    ))
    .expect("Example requires a keypair file");

    let program = client.program(Pubkey::from_str(NFT_MINT_CONTRACT).unwrap()); //9HiRJw3dYo2MV9B1WrqFfoNjWRPS19mjVDCPqAxuMPfb
                                                                                //todo: payer不用再获取，最后也会默认签名
    let to_wallet = program.payer();
    let payer_key = program.payer();

    let nft_mint_key = Keypair::new();
    println!("nft mint key {}", nft_mint_key.pubkey().to_string());

    //当前记忆碎皮的集合的meta_account,权限已经给了付鸿
    //let memorise_mint_account = "6P64iPbit6iUbwMj55pXXEu7GxUaE9jPVqWCmomyqPph";

    let nft_token_account = get_associated_token_address(&to_wallet, &nft_mint_key.pubkey());
    let receiver_token_account =
        get_associated_token_address(&wallet3.pubkey(), &nft_mint_key.pubkey());
    let metadata_address = find_metadata_pda(&nft_mint_key.pubkey());
    let edition_address = find_master_edition_pda(&nft_mint_key.pubkey());

    let account_init_build = program
        .request()
        .accounts(nft_accounts::AccountInit {
            mint: nft_mint_key.pubkey(),
            token_account: nft_token_account,
            mint_authority: payer_key,
            rent: Pubkey::from_str(SYSTEM_RENT_ID).unwrap(),
            system_program: Pubkey::from_str(SYSTEM_PROGRAM_ID).unwrap(),
            token_program: Pubkey::from_str(SPL_PROGRAM_ID).unwrap(),
            associated_token_program: Pubkey::from_str(SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID)
                .unwrap(),
        })
        .args(nft_instructions::AccountInit);

    let mint_build = program
        .request()
        .accounts(nft_accounts::MintNFT{
            metadata: metadata_address, //
            master_edition: edition_address, //
            mint: nft_mint_key.pubkey(), //
            token_account: nft_token_account, //
            mint_authority: payer_key, //
            rent: Pubkey::from_str(SYSTEM_RENT_ID).unwrap(), //
            system_program: Pubkey::from_str(SYSTEM_PROGRAM_ID).unwrap(), //
            token_program: Pubkey::from_str(SPL_PROGRAM_ID).unwrap(), //
            token_metadata_program: Pubkey::from_str(MPL_TOKEN_METADATA_ACCOUNT).unwrap()
        })
        .args(nft_instructions::MintTo{
            title: "test1".to_string(),
            uri: "https://bafybeiagelxwxuundel3rjqydvunf24llrg4e2a5l4fje27arsdzhdgaqu.ipfs.nftstorage.link/0.json".to_string(),
            symbol: "KR".to_string()
        });

    //fixme: 目前不能receiver不能是自己,因为合约里面有创建receiver ata的逻辑
    let verify_build = program
        .request()
        .accounts(nft_accounts::SetAndVerifyCollection {
            metadata_account: find_metadata_pda(&nft_mint_key.pubkey()),
            collection_authority: payer_key,
            payer: payer_key,
            update_authority: payer_key,
            collection_mint: Pubkey::from_str(MEM_COLLECTION_MINT).unwrap(),
            collection_metadata: find_metadata_pda(&Pubkey::from_str(MEM_COLLECTION_MINT).unwrap()),
            collection_master_edition: find_master_edition_pda(
                &Pubkey::from_str(MEM_COLLECTION_MINT).unwrap(),
            ),
            system_program: Pubkey::from_str(SYSTEM_PROGRAM_ID).unwrap(),
            rent: Pubkey::from_str(SYSTEM_RENT_ID).unwrap(),
            spl_token_metadata: Pubkey::from_str(MPL_TOKEN_METADATA_ACCOUNT).unwrap(),

            //for transfer
            mint_account: nft_mint_key.pubkey(),
            sender_token_account: nft_token_account,
            receiver_token_account: receiver_token_account,
            receiver_wallet: wallet3.pubkey(),
            associated_token_program: Pubkey::from_str(SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID)
                .unwrap(),
            spl_token_program: Pubkey::from_str(SPL_PROGRAM_ID).unwrap(),
        })
        .args(nft_instructions::CollectionAdd);

    let mint_res = program
        .request()
        .instruction(
            account_init_build
                .instructions()?
                .first()
                .unwrap()
                .to_owned(),
        )
        .instruction(mint_build.instructions()?.first().unwrap().to_owned())
        .instruction(verify_build.instructions()?.first().unwrap().to_owned())
        .signer(&nft_mint_key)
        .send()?;
    println!("call res {}", mint_res);
    println!("nft mint key {}", nft_mint_key.pubkey().to_string());

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
        .unwrap();

    Ok(nft_mint_key.pubkey())
}
