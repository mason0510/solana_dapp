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
use spl_token::instruction::AuthorityType;
use spl_token::state::{Account, AccountState};


use token_middleware::instruction as token_middleware_instructions;
use token_middleware::accounts as token_middleware_accounts;


use crate::utils::{find_master_edition_pda, find_metadata_pda};
use crate::{MEM_COLLECTION_MINT, MPL_TOKEN_METADATA_ACCOUNT, NFT_MINT_CONTRACT, SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID, SPL_PROGRAM_ID, SYSTEM_PROGRAM_ID, SYSTEM_RENT_ID, TOKEN_MIDDLEWARE};

//可以替第三方铸造
pub fn mint() -> Result<Pubkey> {
    let client = crate::get_wallet("~/.config/solana/id.json".to_string());
    let authority_keypair = read_keypair_file(&*shellexpand::tilde(
        "/Users/eddy/work/repo/solana/solana_dapp/my_wallet/3.json",
    ))
    .expect("Example requires a keypair file");

    let program = client.program(Pubkey::from_str(TOKEN_MIDDLEWARE).unwrap());
    let mint_key = Keypair::new();
    println!("nft mint key {}", mint_key.pubkey().to_string());


    let user_ata = get_associated_token_address(&program.payer(), &mint_key.pubkey());
    let metadata_address = find_metadata_pda(&mint_key.pubkey());
    let master_key = find_master_edition_pda(&mint_key.pubkey());

    //println!("{},{},{},{},{},{}", metadata_address, user_ata, mint_key.pubkey(), authority_keypair.pubkey(), payer_key, master_key);
    let now = format!("timestamp_{}",timestamp() % 100000 );
    let mint_build = program
        .request()
        .accounts(token_middleware_accounts::NftMint{
            authority: authority_keypair.pubkey(),
            metadata: metadata_address,
            user_ata,
            mint: mint_key.pubkey(),
            minter: program.payer(),
            rent: Pubkey::from_str(SYSTEM_RENT_ID).unwrap(),
            system_program: Pubkey::from_str(SYSTEM_PROGRAM_ID).unwrap(),
            token_program: Pubkey::from_str(SPL_PROGRAM_ID).unwrap(),
            token_metadata_program: Pubkey::from_str(MPL_TOKEN_METADATA_ACCOUNT).unwrap(),
            associated_token_program: Pubkey::from_str(SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID).unwrap(),
        })
        .args(token_middleware_instructions::NftMint{
            name: now,
            uri: "https://bafybeiagelxwxuundel3rjqydvunf24llrg4e2a5l4fje27arsdzhdgaqu.ipfs.nftstorage.link/0.json".to_string(),
            collection:None
        });

    let mint_res = program
        .request()
        .instruction(mint_build.instructions()?.first().unwrap().to_owned())
        .signer(&mint_key)
        .send()?;
    println!("call res {}", mint_res);
    println!("nft mint key {}", mint_key.pubkey().to_string());

    Ok(mint_key.pubkey())
}




pub fn mint_master_edition() -> Result<Pubkey> {
    let client = crate::get_wallet("/Users/eddy/work/repo/solana/solana_dapp/my_wallet/3.json".to_string());
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
            name: now,
            uri: "https://bafybeiagelxwxuundel3rjqydvunf24llrg4e2a5l4fje27arsdzhdgaqu.ipfs.nftstorage.link/0.json".to_string(),
        });

    //fixme: 目前不能直接设置collection信息
/*    let collection_mint_key = Pubkey::from_str("2TDavXVuoknovjmVTyiUPaBdQGnTB7q4sJZK1yN7AGd5").unwrap();
    let add_collection_build = program
        .request()
        .accounts(token_middleware_accounts::NftAddCollection{
            collection_authority: program.payer(),
            metadata: metadata_address,
            collection_mint: collection_mint_key,
            collection_metadata: find_metadata_pda(&collection_mint_key),
            collection_master_edition: find_master_edition_pda(&collection_mint_key),
            system_program: Pubkey::from_str(SYSTEM_PROGRAM_ID).unwrap(),
            mpl_token_metadata: Pubkey::from_str(MPL_TOKEN_METADATA_ACCOUNT).unwrap(),
            token_program: Pubkey::from_str(SPL_PROGRAM_ID).unwrap(),
        })
        .args(token_middleware_instructions::NftAddCollection);*/

    let mint_res = program
        .request()
        .instruction(mint_build.instructions()?.first().unwrap().to_owned())
        .signer(&nft_mint_key)
        .send()?;
    println!("call res {}", mint_res);
    println!("nft mint key {}", nft_mint_key.pubkey().to_string());

    Ok(nft_mint_key.pubkey())
}

fn transfer_by_spl(){
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

pub fn burn() -> Result<()>{
    let mint_key = Pubkey::from_str("54z1N9Ef3T2tKsVzs3sV9jwyvxLTi1aF8SXa2q55kELP").unwrap();
    let client = crate::get_wallet("~/.config/solana/id.json".to_string());
    let program = client.program(Pubkey::from_str(TOKEN_MIDDLEWARE).unwrap());
    let minter_key = program.payer();
    println!("nft mint key {}", mint_key.to_string());
    let user_ata = get_associated_token_address(&Pubkey::from_str("677NzkzkDKT9wXDMXGPUvbFp1T7XzJtZZxcRaBAaSvNa").unwrap(), &mint_key);
    println!("user_ata key {}", user_ata.to_string());

    let mint_build = program
        .request()
        .accounts(token_middleware_accounts::NftBurn{
            authority: program.payer(),
            mint: mint_key,
            user_ata,
            system_program: Pubkey::from_str(SYSTEM_PROGRAM_ID).unwrap(),
            token_program: Pubkey::from_str(SPL_PROGRAM_ID).unwrap(),
        })
        .args(token_middleware_instructions::NftBurn);

    let freeze_res = program
        .request()
        .instruction(mint_build.instructions()?.first().unwrap().to_owned())
        .send()?;
    println!("call res {}", freeze_res);
    println!("nft mint key {}", mint_key.to_string());
    Ok(())
}
pub fn freeze(client: &Client,mint_key: Pubkey) -> Result<()>{
    let wallet3 = read_keypair_file(&*shellexpand::tilde("/Users/eddy/work/repo/solana/solana_dapp/my_wallet/3.json", )).unwrap();
    let program = client.program(Pubkey::from_str(TOKEN_MIDDLEWARE).unwrap());
    let to= Pubkey::from_str("677NzkzkDKT9wXDMXGPUvbFp1T7XzJtZZxcRaBAaSvNa").unwrap();
    let minter_key = program.payer();
    println!("nft mint key {}", mint_key.to_string());
    let user_ata = get_associated_token_address(&to, &mint_key);

    /***
    pub authority: Signer<'info>,
    pub mint: UncheckedAccount<'info>,
    pub user_ata: UncheckedAccount<'info>,
    pub mint_owner: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
nft_freeze
    */
    let mint_build = program
        .request()
        .accounts(token_middleware_accounts::NftFreeze{
            authority: wallet3.pubkey(),
            mint: mint_key,
            user_ata,
            mint_owner: minter_key,
            system_program: Pubkey::from_str(SYSTEM_PROGRAM_ID).unwrap(),
            token_program: Pubkey::from_str(SPL_PROGRAM_ID).unwrap(),
        })
        .args(token_middleware_instructions::NftFreeze);

    let freeze_res = program
        .request()
        .instruction(mint_build.instructions()?.first().unwrap().to_owned())
        .signer(&wallet3)
        .send()?;
    println!("call res {}", freeze_res);
    println!("nft mint key {}", mint_key.to_string());

    let user_ata = program.rpc().get_account(&user_ata.key()).unwrap();//connget_account(user_ata.key())
    let account = Account::unpack_unchecked(&user_ata.data).unwrap();
    println!("call res {:?}", account.state);
    assert_eq!(account.state, AccountState::Frozen);
    Ok(())
}
fn thaw(){
    todo!()
}

pub fn add_collection(mint_key: Pubkey,collection_mint: Pubkey) -> Result<()>{
    let client = crate::get_wallet("/Users/eddy/work/repo/solana/solana_dapp/my_wallet/3.json".to_string());
    let program = client.program(Pubkey::from_str(TOKEN_MIDDLEWARE).unwrap());
    println!("nft mint key {}", mint_key.to_string());
    let metadata_key = find_metadata_pda(&mint_key);
    let mint_build = program
        .request()
        .accounts(token_middleware_accounts::NftAddCollection{
            collection_authority: program.payer(),
            metadata: metadata_key,
            collection_mint,
            collection_metadata: find_metadata_pda(&collection_mint),
            collection_master_edition: find_master_edition_pda(&collection_mint),
            system_program: Pubkey::from_str(SYSTEM_PROGRAM_ID).unwrap(),
            mpl_token_metadata: Pubkey::from_str(MPL_TOKEN_METADATA_ACCOUNT).unwrap(),
            token_program: Pubkey::from_str(SPL_PROGRAM_ID).unwrap(),
        })
        .args(token_middleware_instructions::NftAddCollection);

    let freeze_res = program
        .request()
        .instruction(mint_build.instructions()?.first().unwrap().to_owned())
        .send()?;
    println!("call res {}", freeze_res);
    println!("nft mint key {}", mint_key.to_string());

    let user_ata = program.rpc().get_account(&metadata_key).unwrap();//connget_account(user_ata.key())
    let data1 = solana_sdk::borsh::try_from_slice_unchecked::<mpl_token_metadata::state::Metadata>(
        &user_ata.data.as_slice(),
    ).unwrap();
    println!("call res {:?}", data1.data);
    //todo
    //assert_eq!(data1.data.creators.unwrap().first().unwrap().verified, true);
    //assert_eq!(data1.data.creators.unwrap().first().unwrap().address, wallet3.pubkey());
    assert_eq!(data1.collection.as_ref().unwrap().verified, true);
    assert_eq!(data1.collection.as_ref().unwrap().key, collection_mint);

    Ok(())
}

pub fn transfer() -> Result<()>{
    let to= Pubkey::from_str("EpGFtdBwTB5BRJZRS98wapNugb4eGjrAQFrBYphCLZMd").unwrap();
    let mint = Pubkey::from_str("2TDavXVuoknovjmVTyiUPaBdQGnTB7q4sJZK1yN7AGd5").unwrap();
    let client = crate::get_wallet("/Users/eddy/work/repo/solana/solana_dapp/my_wallet/3.json".to_string());
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

pub fn update_meta() -> Result<()>{
    let mint = Pubkey::from_str("Gua9n1w2rLVfyNzVpZDZv9i1YDRUovDdFybpj5FchdX1").unwrap();
    let client = crate::get_wallet("/Users/eddy/work/repo/solana/solana_dapp/my_wallet/3.json".to_string());
    let program = client.program(Pubkey::from_str(TOKEN_MIDDLEWARE).unwrap());
    let mint_build = program
        .request()
        .accounts(token_middleware_accounts::NftUpdateMeta{
            authority: program.payer(),
            metadata: find_metadata_pda(&mint),
            mpl_token_metadata: Pubkey::from_str(MPL_TOKEN_METADATA_ACCOUNT).unwrap(),
        })
        .args(token_middleware_instructions::NftUpdateMeta{
            name: Some("".to_string()),
            uri: Some("".to_string()),
        });

    let mint_res = program
        .request()
        .instruction(mint_build.instructions()?.first().unwrap().to_owned())
        .send()?;
    println!("call res {}", mint_res);
    Ok(())
}


pub fn update_authority() -> Result<()>{
    //Abt6HCQQpCxrKzmrhE1rM7p9RLpCMYA5Q3om2zvLM91n
    let mint_key = Pubkey::from_str("8FLC5ETUx3LB3u9PBKqaJ3wJVzSBw6cG4csLr171Eieo").unwrap();
    let new_authrity = Pubkey::from_str("Abt6HCQQpCxrKzmrhE1rM7p9RLpCMYA5Q3om2zvLM93n").unwrap();
    let client = crate::get_wallet("/Users/eddy/work/repo/solana/solana_dapp/my_wallet/3.json".to_string());
    let program = client.program(Pubkey::from_str(TOKEN_MIDDLEWARE).unwrap());
    let user_ata = get_associated_token_address(&program.payer(), &mint_key);
    let mint_res = program
        .request()
        .instruction(spl_token::instruction::set_authority(
            &Pubkey::from_str(SPL_PROGRAM_ID).unwrap(),
            &mint_key,
            Some(&new_authrity),
            AuthorityType::FreezeAccount,
            &program.payer(),
            &[
                &program.payer()
            ])?)
        .send()?;
    println!("call res {}", mint_res);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn transfer_test(){
        //nft::transfer().unwrap();
    }
    #[test]
    fn mint_test(){
        //nft::mint().unwrap();
    }
    #[test]
    fn add_collection_test(){
        //    nft::add_collection(mint_key,Pubkey::from_str("2TDavXVuoknovjmVTyiUPaBdQGnTB7q4sJZK1yN7AGd5").unwrap())
    }
    #[test]
    fn update_metadata_test(){
        //nft::update_meta().unwrap();
    }
    #[test]
    fn mint_master_edition(){
        //mint_master_edition().unwrap();
    }
    //nft::freeze(&get_wallet("/Users/eddy/work/repo/solana/solana_dapp/my_wallet/3.json".to_string()),
    //Pubkey::from_str("7g7EMgCoX7x5GV69w59VRCWaJwTj6NxLvBcQi7dxAJYG").unwrap()).unwrap();

    #[test]
    fn burn_test(){
        //nft::burn().unwrap();
    }

    #[test]
    fn freeze_test(){
        //    nft::freeze(client,mint_key)
    }
}