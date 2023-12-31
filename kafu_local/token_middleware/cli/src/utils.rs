extern crate core;

use anchor_client::solana_sdk::commitment_config::CommitmentConfig;

use anchor_client::ClientError;

use solana_account_decoder::UiAccountEncoding;
use solana_client::{
    rpc_client::RpcClient,
    rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
    rpc_filter::{Memcmp, MemcmpEncodedBytes, MemcmpEncoding, RpcFilterType},
};
use solana_sdk::program_pack::Pack;
use spl_token::state::Account;

// The `accounts` and `instructions` modules are generated by the framework.
//use events::instruction as events_instruction;
//use events::MyEvent;

// The `accounts` and `instructions` modules are generated by the framework.
//use composite::accounts::{Bar, CompositeUpdate, Foo, Initialize};
//use composite::instruction as composite_instruction;
//use composite::{DummyA, DummyB};

use std::str::FromStr;

use anchor_client::anchor_lang::prelude::Pubkey;

use mpl_token_metadata::pda::{find_master_edition_account, find_metadata_account};

#[cfg(feature = "serde-feature")]
use {
    serde::{Deserialize, Serialize},
    serde_with::{As, DisplayFromStr},
};

//import { TOKEN_PROGRAM_ID, createAssociatedTokenAccountInstruction, getAssociatedTokenAddress, createInitializeMintInstruction, MINT_SIZE } from '@solana/spl-token' // IGNORE THESE ERRORS IF ANY

use solana_sdk::account::ReadableAccount;

use crate::SPL_PROGRAM_ID;

pub fn find_metadata_pda(mint: &Pubkey) -> Pubkey {
    let (pda, _bump) = find_metadata_account(mint);

    pda
}

pub fn find_master_edition_pda(mint: &Pubkey) -> Pubkey {
    let (pda, _bump) = find_master_edition_account(mint);
    pda
}

pub fn get_acc(address: Pubkey) -> solana_sdk::account::Account {
    let rpc_client = RpcClient::new("https://api.devnet.solana.com".to_string());
    rpc_client
        .get_account_with_commitment(&address, CommitmentConfig::processed())
        .unwrap()
        .value
        .ok_or(ClientError::AccountNotFound)
        .unwrap()
}

//deprecated：通过find_program_address或者类似get_associated_token_address的接口查找pda，判断是否存在
pub fn get_token_account_by_wallet(wallet_pubkey: Pubkey, mint_pubkey: Pubkey) -> Option<Pubkey> {
    let rpc_url = String::from("https://api.devnet.solana.com");
    let connection = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let filters = Some(vec![
        RpcFilterType::Memcmp(Memcmp {
            offset: 32,
            bytes: MemcmpEncodedBytes::Base58(wallet_pubkey.to_string()),
            encoding: Some(MemcmpEncoding::Binary),
        }),
        RpcFilterType::DataSize(165),
    ]);

    let accounts = connection
        .get_program_accounts_with_config(
            &Pubkey::from_str(SPL_PROGRAM_ID).unwrap(),
            RpcProgramAccountsConfig {
                filters,
                account_config: RpcAccountInfoConfig {
                    encoding: Some(UiAccountEncoding::Base64),
                    commitment: Some(connection.commitment()),
                    ..RpcAccountInfoConfig::default()
                },
                ..RpcProgramAccountsConfig::default()
            },
        )
        .unwrap();

    println!(
        "Found {:?} token account(s) for wallet {}: ",
        accounts.len(),
        wallet_pubkey.to_string()
    );
    let token_account = accounts.iter().find(|&account| {
        let mint_token_account = Account::unpack_from_slice(account.1.data.as_slice()).unwrap();
        let mint_token_account = mint_token_account.mint.to_string();
        mint_pubkey.to_string() == mint_token_account
    });
    //todo: 不仅检查是否在当前program找到，还要检查所有的
    token_account.map(|token_account| token_account.0.to_owned())
}

fn update_metadata() {
    todo!()
}

fn get_lamport_balance() {
    todo!()
}

fn get_spl_token_balance() {
    todo!()
}
