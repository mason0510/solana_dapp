use anchor_client::solana_sdk::signature::{Keypair, Signer};
use anchor_client::{Client, Cluster};
use anyhow::Result;
use std::str::FromStr;
use anchor_client::anchor_lang::Key;
use anchor_client::anchor_lang::prelude::Pubkey;
use anchor_client::anchor_lang::solana_program::program_pack::Pack;
use borsh::BorshDeserialize;
use mpl_token_metadata::state::{Collection, Metadata, TokenMetadataAccount};
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


pub fn mint() -> Result<Pubkey> {
    let client = crate::get_wallet("/Users/eddy/work/repo/solana/solana_dapp/my_wallet/3.json".to_string());
    let program = client.program(Pubkey::from_str(TOKEN_MIDDLEWARE).unwrap());
    let coin_key = Keypair::new();
    println!("nft mint key {}", coin_key.pubkey().to_string());

    let user_ata = get_associated_token_address(&program.payer(), &coin_key.pubkey());
    let metadata_address = find_metadata_pda(&coin_key.pubkey());
    let master_key = find_master_edition_pda(&coin_key.pubkey());

    let now = format!("kcoin_{}",timestamp() % 100000 );
    let mint_build = program
        .request()
        .accounts(token_middleware_accounts::CoinMint{
            metadata: metadata_address,
            user_ata,
            mint: coin_key.pubkey(),
            minter: program.payer(),
            rent: Pubkey::from_str(SYSTEM_RENT_ID).unwrap(),
            system_program: Pubkey::from_str(SYSTEM_PROGRAM_ID).unwrap(),
            token_program: Pubkey::from_str(SPL_PROGRAM_ID).unwrap(),
            token_metadata_program: Pubkey::from_str(MPL_TOKEN_METADATA_ACCOUNT).unwrap(),
            associated_token_program: Pubkey::from_str(SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID).unwrap(),
        })
        .args(token_middleware_instructions::CoinMint{
            symbol: "K".to_string(),
            name: "Kin".to_string(),
            uri: "https://bafybeihome3tmx7xdhfj6nt63pttafzmjchzib4kgcpdotj22bnczp53ji.ipfs.nftstorage.link/kin.json".to_string(),
            init_supply: 100_000_000u64 * 10u64.pow(9)
        });

    let mint_res = program
        .request()
        .instruction(mint_build.instructions()?.first().unwrap().to_owned())
        .signer(&coin_key)
        .send()?;
    println!("call res {}", mint_res);
    println!("coin mint key {}", coin_key.pubkey().to_string());

    Ok(coin_key.pubkey())
}

//must be the mint authority
pub fn issue(coin_pub_key: Pubkey) -> Result<()>{
    let client = crate::get_wallet("/Users/eddy/work/repo/solana/solana_dapp/my_wallet/3.json".to_string());
    let program = client.program(Pubkey::from_str(TOKEN_MIDDLEWARE).unwrap());
    let user_ata = get_associated_token_address(&program.payer(), &coin_pub_key);
    let now = format!("kcoin_{}",solana_sdk::timing::timestamp() % 100000 );
    let mint_build = program
        .request()
        .accounts(token_middleware_accounts::CoinIssue{
            user_ata,
            mint: coin_pub_key,
            minter: program.payer(),
            token_program: Pubkey::from_str(SPL_PROGRAM_ID).unwrap(),
        })
        .args(token_middleware_instructions::CoinIssue{
            amount: 1_000_000u64 * 10u64.pow(9)
        });

    let mint_res = program
        .request()
        .instruction(mint_build.instructions()?.first().unwrap().to_owned())
        .send()?;
    println!("call res {}", mint_res);
    Ok(())
}

pub fn transfer() -> Result<()>{
    let to= Pubkey::from_str("9EZZmeAE16RsKPxbL9VXBjGFooPsKfePRxfyLJrp8umu").unwrap();
    let amount = 12345678u64 * 10u64.pow(9);
    let coin_pub_key = Pubkey::from_str("Ba5cuVSN5p2BwNxfFJR74yLaE61q3ugUmKWxnRtirVFi").unwrap();

    let client = crate::get_wallet("/Users/eddy/work/repo/solana/solana_dapp/my_wallet/3.json".to_string());
    //todo: 这个也封装到get_wallet里面
    let program = client.program(Pubkey::from_str(TOKEN_MIDDLEWARE).unwrap());
    let from_ata = get_associated_token_address(&program.payer(), &coin_pub_key);
    let to_ata = get_associated_token_address(&to, &coin_pub_key);


    let mint_build = program
        .request()
        .accounts(token_middleware_accounts::CoinTransfer{
            from_ata,
            from: program.payer(),
            to,
            to_ata,
            coin: coin_pub_key,
            token_program: Pubkey::from_str(SPL_PROGRAM_ID).unwrap(),
            associated_token_program: Pubkey::from_str(SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID).unwrap(),
            system_program: Pubkey::from_str(SYSTEM_PROGRAM_ID).unwrap(),
            rent:Pubkey::from_str(SYSTEM_RENT_ID).unwrap(),
        })
        .args(token_middleware_instructions::CoinTransfer{
            amount
        });

    let mint_res = program
        .request()
        .instruction(mint_build.instructions()?.first().unwrap().to_owned())
        .send()?;
    println!("call res {}", mint_res);
    Ok(())
}

pub fn update_icon() -> Result<()>{
    let coin_mint = Pubkey::from_str("Ba5cuVSN5p2BwNxfFJR74yLaE61q3ugUmKWxnRtirVFi").unwrap();
    let coin_metadata = find_metadata_pda(&coin_mint);
    let client = crate::get_wallet("/Users/eddy/work/repo/solana/solana_dapp/my_wallet/3.json".to_string());
    let program = client.program(Pubkey::from_str(TOKEN_MIDDLEWARE).unwrap());
    let test1 = program.rpc().get_account(&coin_metadata).unwrap().data;
    let mut old_data = solana_sdk::borsh::try_from_slice_unchecked::<Metadata, >(test1.as_slice()).unwrap().data;
    old_data.uri = "https://bafybeiaybw2u4vfama3n6j6r2rehqmvarysruzqdo4humlvaejd5wwbuae.ipfs.nftstorage.link/kin.json".to_string();
    let update_ins = mpl_token_metadata::instruction::update_metadata_accounts(
        Pubkey::from_str(MPL_TOKEN_METADATA_ACCOUNT).unwrap(),
        coin_metadata,
        program.payer(),
        None,
        Some(old_data),
        None);
    let mint_res = program
        .request()
        .instruction(update_ins)
        .send()?;
    println!("call res {}", mint_res);
    Ok(())
}


pub fn batch_transfer() -> Result<()>{
    let to= Pubkey::from_str("677NzkzkDKT9wXDMXGPUvbFp1T7XzJtZZxcRaBAaSvNa").unwrap();
    let amount = 1 * 10u64.pow(9);
    let coin_pub_key = Pubkey::from_str("Ba5cuVSN5p2BwNxfFJR74yLaE61q3ugUmKWxnRtirVFi").unwrap();

    let client = crate::get_wallet("/Users/eddy/work/repo/solana/solana_dapp/my_wallet/3.json".to_string());
    //todo: 这个也封装到get_wallet里面
    let program = client.program(Pubkey::from_str(TOKEN_MIDDLEWARE).unwrap());
    let from_ata = get_associated_token_address(&program.payer(), &coin_pub_key);
    let to_ata = get_associated_token_address(&to, &coin_pub_key);


    let mint_build = program
        .request()
        .accounts(token_middleware_accounts::CoinTransfer{
            from_ata,
            from: program.payer(),
            to,
            to_ata,
            coin: coin_pub_key,
            token_program: Pubkey::from_str(SPL_PROGRAM_ID).unwrap(),
            associated_token_program: Pubkey::from_str(SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID).unwrap(),
            system_program: Pubkey::from_str(SYSTEM_PROGRAM_ID).unwrap(),
            rent:Pubkey::from_str(SYSTEM_RENT_ID).unwrap(),
        })
        .args(token_middleware_instructions::CoinTransfer{
            amount
        });

    //最多28个
    let transfer_ins =  mint_build.instructions()?.first().unwrap().to_owned();
    let mint_res = program
        .request()
        .instruction(transfer_ins.clone())
        .instruction(transfer_ins.clone())
        .instruction(transfer_ins.clone())
        .send()?;
    println!("call res {}", mint_res);
    Ok(())
}

/***

    //test_add_collection()?;
    //coin::mint().unwrap();
    //coin::issue(Pubkey::from_str("87p3mq7h69UwJnkwsEio84tbM8DXyTg6rx3SJxhRF5st").unwrap());
    //coin::transfer().unwrap();
    //coin::transfer().unwrap();
*/
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
    fn issue_test(){
        //    nft::add_collection(mint_key,Pubkey::from_str("2TDavXVuoknovjmVTyiUPaBdQGnTB7q4sJZK1yN7AGd5").unwrap())
    }
    #[test]
    fn update_metadata_test(){
        //nft::update_meta().unwrap();
    }

    #[test]
    fn freeze_test(){
        //    nft::freeze(client,mint_key)
    }

    #[test]
    fn thaw_test(){
        //    nft::freeze(client,mint_key)
    }
}