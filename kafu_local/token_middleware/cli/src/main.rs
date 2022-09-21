pub mod buy;
pub mod cancel;
pub mod nft;
pub mod sell;
pub mod settings;
pub mod utils;
extern crate core;

use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use anchor_client::solana_sdk::signature::read_keypair_file;

use anchor_client::{Client, Cluster};

use anyhow::Result;

// The `accounts` and `instructions` modules are generated by the framework.
//use events::instruction as events_instruction;
//use events::MyEvent;
use clap::Parser;
// The `accounts` and `instructions` modules are generated by the framework.
//use composite::accounts::{Bar, CompositeUpdate, Foo, Initialize};
//use composite::instruction as composite_instruction;
//use composite::{DummyA, DummyB};

use std::rc::Rc;
use std::str::FromStr;

use anchor_client::anchor_lang::prelude::Pubkey;

#[cfg(feature = "serde-feature")]
use {
    serde::{Deserialize, Serialize},
    serde_with::{As, DisplayFromStr},
};
use escrow_marketplace::constants::{ESCROW_INFO, MARKET_SETTING};

//import { TOKEN_PROGRAM_ID, createAssociatedTokenAccountInstruction, getAssociatedTokenAddress, createInitializeMintInstruction, MINT_SIZE } from '@solana/spl-token' // IGNORE THESE ERRORS IF ANY

use crate::sell::list_orders;

#[derive(Parser, Debug)]
pub struct Opts {
    #[clap(long)]
    pub token_address: Pubkey,
    #[clap(long)]
    pub bridge_contract_pid: Pubkey,
    #[clap(long)]
    pub receiver_wallet: Pubkey,
}
//K coin
const K_COIN: &'static str = "5d1i4wKHhGXXkdZB22iKD1SqU6pkBeTCwFEMqo7xy39h";

const SPL_PROGRAM_ID: &'static str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
//市场托管合约
const ESCROW_MARKETPLACE: &'static str = "D8yTyPU9tSvJc8EuaUqRcvYsAj6SuPoYFg1uZG6istQB";
//一键生成NFT的合约
const NFT_MINT_CONTRACT: &'static str = "9HiRJw3dYo2MV9B1WrqFfoNjWRPS19mjVDCPqAxuMPfb";
const SENDER: &'static str = "9hUYW9s2c98GfjZb6JvW62BYEt3ryxGmeMBkhgSqmZtW";
const SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID: &'static str =
    "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";
const SYSTEM_PROGRAM_ID: &'static str = "11111111111111111111111111111111";
const SYSTEM_RENT_ID: &'static str = "SysvarRent111111111111111111111111111111111";
const MPL_TOKEN_METADATA_ACCOUNT: &'static str = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s";
const MEM_COLLECTION_MINT: &'static str = "8zKSXBACKpaKvgDCYdDwpJGTVDSBCtAgucJpmR7gAyx5";

// This example assumes a local validator is running with the programs
// deployed at the addresses given by the CLI args.

fn test_sell(client: &Client) {
    let nft_mint_key = nft::simple_mint(client).unwrap();
    list_orders();
    sell::sell4lamport(client, nft_mint_key);
    list_orders();
}
fn test_sell_and_cancel(client: &Client) {
    let nft_mint_key = nft::simple_mint(client).unwrap();
    sell::sell4lamport(client, nft_mint_key);
    list_orders();
    cancel::cancel(client, nft_mint_key);
    list_orders();
}
fn test_sell_and_buy(client: &Client) {
    let nft_mint_key = nft::simple_mint(client).unwrap();
    list_orders();
    sell::sell4lamport(client, nft_mint_key);
    buy::buy_and_pay_lamport(client, nft_mint_key);
    list_orders();
}
// only once for a contract
fn test_init_setting(_client: &Client) {
    todo!()
}

// only once for a contract
fn test_update_settings(_client: &Client) {
    todo!()
}

//some assert
fn test_not_support_sell() {
    todo!()
}

fn main() -> Result<()> {
    println!("Starting test...");
    //replace with fix code
    let _opts = Opts::parse();

    // Wallet and cluster params.
    let payer = read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json"))
        .expect("Example requires a keypair file");
    let url = Cluster::Custom(
        "https://api.devnet.solana.com".to_string(),
        "wss://api.devnet.solana.com/".to_string(),
    );

    // Client.
    let client = Client::new_with_options(url, Rc::new(payer), CommitmentConfig::processed());
    //test_sell_and_cancel(&client);
    //test_sell_and_buy(&client);
    //list_orders();
    //test_sell(&client);
    let mint_key = Pubkey::from_str("BrrRtYALnoxsqQARmJ2ThnQr7o1nrsZiNNhfNMHkbspC").unwrap();
    let (market_setting_pda, _) = Pubkey::find_program_address(
        &[ESCROW_INFO,mint_key.as_ref()
        ],
        &Pubkey::from_str(ESCROW_MARKETPLACE).unwrap(),
    );
    println!("{}",market_setting_pda.to_string());
    Ok(())
}
