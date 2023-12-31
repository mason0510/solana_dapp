extern crate core;

use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use anchor_client::solana_sdk::signature::read_keypair_file;
use anchor_client::solana_sdk::signature::Signer;

use anchor_client::Client;

use solana_client::rpc_client::RpcClient;

// The `accounts` and `instructions` modules are generated by the framework.
//use events::instruction as events_instruction;
//use events::MyEvent;

// The `accounts` and `instructions` modules are generated by the framework.
//use composite::accounts::{Bar, CompositeUpdate, Foo, Initialize};
//use composite::instruction as composite_instruction;
//use composite::{DummyA, DummyB};

use std::str::FromStr;

use anchor_client::anchor_lang::prelude::Pubkey;

#[cfg(feature = "serde-feature")]
use {
    serde::{Deserialize, Serialize},
    serde_with::{As, DisplayFromStr},
};

//import { TOKEN_PROGRAM_ID, createAssociatedTokenAccountInstruction, getAssociatedTokenAddress, createInitializeMintInstruction, MINT_SIZE } from '@solana/spl-token' // IGNORE THESE ERRORS IF ANY

use escrow_marketplace::accounts as market_accounts;
use escrow_marketplace::instruction as market_instructions;

use escrow_marketplace::constants::MARKET_SETTING;
use solana_sdk::account::ReadableAccount;

use crate::{ESCROW_MARKETPLACE, SYSTEM_PROGRAM_ID};
use escrow_marketplace::state::order::Settings;

fn init_settings(client: &Client) {
    let program = client.program(Pubkey::from_str(ESCROW_MARKETPLACE).unwrap());
    let _authority = program.payer();
    let payer = read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json"))
        .expect("Example requires a keypair file");

    let (market_setting_pda, _) = Pubkey::find_program_address(
        &[MARKET_SETTING],
        &Pubkey::from_str(ESCROW_MARKETPLACE).unwrap(),
    );

    let init_res = program
        .request()
        .accounts(market_accounts::InitSettings {
            setting_account: market_setting_pda,
            authority: payer.pubkey(),
            system_program: Pubkey::from_str(SYSTEM_PROGRAM_ID).unwrap(),
        })
        .args(market_instructions::InitSettings {
            support_coins: vec![],
            fee_ratio: 100, //1%
        })
        .signer(&payer)
        .send()
        .unwrap();
    println!("init settings {}", init_res.to_string());
}
fn update_settings(client: &Client) {
    let program = client.program(Pubkey::from_str(ESCROW_MARKETPLACE).unwrap());
    let _authority = program.payer();
    let payer = read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json"))
        .expect("Example requires a keypair file");
    let _buyer = read_keypair_file(&*shellexpand::tilde(
        "/Users/eddy/work/repo/solana/solana_dapp/my_wallet/2.json",
    ))
    .expect("Example requires a keypair file");

    let (market_setting_pda, _) = Pubkey::find_program_address(
        &[MARKET_SETTING],
        &Pubkey::from_str(ESCROW_MARKETPLACE).unwrap(),
    );

    let init_res = program
        .request()
        .accounts(market_accounts::UpdateSettings {
            setting_account: market_setting_pda,
            authority: payer.pubkey(),
        })
        .args(market_instructions::UpdateSettings {
            support_coins: vec![],
            fee_ratio: 123, //1.23%
            new_authority: None,
        })
        .send()
        .unwrap();
    println!("init settings {}", init_res.to_string());
}
fn list_settings() {
    let rpc_url = String::from("https://api.devnet.solana.com");
    let connection = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
    let order_accounts = connection
        .get_program_accounts(&Pubkey::from_str(ESCROW_MARKETPLACE).unwrap())
        .unwrap();
    for (_key, account) in order_accounts.iter() {
        match solana_sdk::borsh::try_from_slice_unchecked::<Settings>(&account.data.as_slice()[8..])
        {
            Ok(data) => {
                println!("find setting data {:#?}", data);
            }
            Err(error) => {
                println!("{:?}", error.to_string());
            }
        }
    }
}
