extern crate core;

use std::cmp::min;
use std::io::Error;
use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use anchor_client::solana_sdk::signature::read_keypair_file;
use anchor_client::solana_sdk::signature::{Keypair, Signer};
use anchor_client::solana_sdk::system_instruction;
use anchor_client::{Client, ClientError, Cluster, EventContext};
use mpl_token_metadata::instruction::{set_and_verify_collection,verify_collection};
use borsh::{BorshDeserialize, BorshSerialize};


use solana_client::{
    rpc_client::RpcClient,
    rpc_filter::{RpcFilterType, Memcmp, MemcmpEncodedBytes, MemcmpEncoding},
    rpc_config::{RpcProgramAccountsConfig, RpcAccountInfoConfig},
};
use solana_sdk::program_pack::Pack;
use spl_token::{state::{Mint, Account}};
use solana_account_decoder::{UiAccountEncoding};

use anyhow::Result;
use solana_sdk::{client, system_program};
// The `accounts` and `instructions` modules are generated by the framework.
//use events::instruction as events_instruction;
//use events::MyEvent;
use clap::Parser;
// The `accounts` and `instructions` modules are generated by the framework.
//use composite::accounts::{Bar, CompositeUpdate, Foo, Initialize};
//use composite::instruction as composite_instruction;
//use composite::{DummyA, DummyB};
use rand::rngs::OsRng;
use std::rc::Rc;
use std::str::FromStr;
use std::time::Duration;
use anchor_client::anchor_lang::Key;
use anchor_client::anchor_lang::prelude::{Pubkey, Sysvar};
use anchor_client::solana_client::nonce_utils::get_account;
use anchor_client::solana_sdk::nonce::State;
use mpl_token_metadata::pda::{find_master_edition_account, find_metadata_account};
use mpl_token_metadata::state::{Metadata, PREFIX, TokenMetadataAccount};
use solana_client::nonce_utils::get_account_with_commitment;
use solana_sdk::account_info::AccountInfo;
use spl_associated_token_account::{create_associated_token_account,get_associated_token_address};
use spl_associated_token_account::solana_program::pubkey;

#[cfg(feature = "serde-feature")]
use {
    serde::{Deserialize, Serialize},
    serde_with::{As, DisplayFromStr},
};

//import { TOKEN_PROGRAM_ID, createAssociatedTokenAccountInstruction, getAssociatedTokenAddress, createInitializeMintInstruction, MINT_SIZE } from '@solana/spl-token' // IGNORE THESE ERRORS IF ANY

use spl_token::instruction::initialize_mint;
use nft_mint_and_verify::instruction as nft_instructions;
use nft_mint_and_verify::accounts as nft_accounts;

use escrow_marketplace::instruction as market_instructions;
use escrow_marketplace::accounts as market_accounts;
use serde::{Deserialize,Serialize};
use solana_sdk::account::ReadableAccount;
use escrow_marketplace::constants::{MARKET_SETTING, ORDER_SIZE, SETTING_SIZE, VAULT_PREFIX, VAULT_SIGNER};

use escrow_marketplace::state::order::{SellOrder, Settings};
use crate::{ESCROW_MARKETPLACE, SPL_PROGRAM_ID, SYSTEM_PROGRAM_ID, SYSTEM_RENT_ID};
use crate::utils::{find_metadata_pda,get_token_account_by_wallet};


pub fn list_orders(){
    let rpc_url = String::from("https://api.devnet.solana.com");
    let connection = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
    let filter_size = RpcFilterType::DataSize(ORDER_SIZE as u64);
    let mut filter_conf = RpcProgramAccountsConfig {
        filters: Some(vec![filter_size]),
        account_config: RpcAccountInfoConfig{
            encoding: Some(UiAccountEncoding::Base64),
            data_slice: None,
            commitment: None
        },
        with_context: None
    };
    let order_accounts = connection.get_program_accounts_with_config(&Pubkey::from_str(ESCROW_MARKETPLACE).unwrap(), filter_conf).unwrap();
    let order_infos = order_accounts.iter().map(|(escrow_key,escrow_account)| {
        //println!("escrow_key  {:?}",escrow_key);
        match solana_sdk::borsh::try_from_slice_unchecked::<SellOrder>(&escrow_account.data.as_slice()[8..]) {
            Ok(data) => {
                let meta_data_account = find_metadata_pda(&data.mint_account);
                //println!("mint {},metadata {}",data.mint_account.to_string(),meta_data_account.to_string());
                let test1 = connection.get_account(&meta_data_account).unwrap().data;
                let meta_data = solana_sdk::borsh::try_from_slice_unchecked::<mpl_token_metadata::state::Metadata>(test1.as_slice()).unwrap();
                Some(meta_data)
            }
            Err(error) =>{
                //原来还没删掉的脏数据
                //println!("{:?}",error.to_string());
                None
            }
        }
    }).collect::<Vec<Option<Metadata>>>();
    println!("all sell orders size {:#?}",order_infos.len());
}

pub fn sell4kcoin(){
    todo!()
}

pub fn sell4lamport(client: &Client, nft_mint_key: Pubkey) -> Pubkey{
    let program = client.program(Pubkey::from_str(ESCROW_MARKETPLACE).unwrap());
    let payer_key = program.payer();
    let escrow_account = Keypair::new();
    let (vault_account_pda, _vault_account_bump) =   Pubkey::find_program_address(
        &[VAULT_PREFIX,nft_mint_key.as_ref()],
        &Pubkey::from_str(ESCROW_MARKETPLACE).unwrap()
    );

    //todo:这里的vault_authority_pda没有用
    let (vault_authority_pda, _escrow_account_bump) =   Pubkey::find_program_address(
        &[VAULT_SIGNER],
        &Pubkey::from_str(ESCROW_MARKETPLACE).unwrap()
    );

    let (market_setting_pda, _) =   Pubkey::find_program_address(
        &[MARKET_SETTING],
        &Pubkey::from_str(ESCROW_MARKETPLACE).unwrap()
    );

    let seller_token_account = get_associated_token_address(&payer_key,&nft_mint_key);
    println!("escrow_account key {},seller_token_account {},vault_authority_pda {}", escrow_account.pubkey().to_string(),seller_token_account.to_string(),vault_authority_pda.to_string());
    println!("nft mint key {},vault_account_pda {}", nft_mint_key.to_string(),vault_account_pda);


    let sell_res = program
        .request()
        .accounts(market_accounts::Sell{
            seller: payer_key,
            nft_mint: nft_mint_key,
            vault_account: vault_account_pda,
            seller_token_account: seller_token_account,
            setting_account: market_setting_pda,
            escrow_account: escrow_account.pubkey(),
            system_program: Pubkey::from_str(SYSTEM_PROGRAM_ID).unwrap(),
            rent: Pubkey::from_str(SYSTEM_RENT_ID).unwrap(),
            token_program: Pubkey::from_str(SPL_PROGRAM_ID).unwrap(),
        })
        .args(market_instructions::Sell{_vault_authority_key:vault_authority_pda,receive_coin: None,price:1_000_000_000})//todo: test pay kcoin
        .signer(&escrow_account)
        .send().unwrap();

    println!("nft mint key {},vault_account_pda {}", nft_mint_key.to_string(),vault_account_pda);
    println!("finished sell lamport, call res {}",sell_res.to_string());
    escrow_account.pubkey()
}