use std::cmp::min;
use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_client::solana_sdk::signature::read_keypair_file;
use anchor_client::solana_sdk::signature::{Keypair, Signer};
use anchor_client::solana_sdk::system_instruction;
use anchor_client::{Client, Cluster, EventContext};

//use basic_2::Counter;

use solana_client::{
    rpc_client::RpcClient,
    rpc_filter::{RpcFilterType, Memcmp, MemcmpEncodedBytes, MemcmpEncoding},
    rpc_config::{RpcProgramAccountsConfig, RpcAccountInfoConfig},
};
use solana_sdk::program_pack::Pack;
use spl_token::{state::{Mint, Account}};
use solana_account_decoder::{UiAccountEncoding};

use anyhow::Result;
use solana_sdk::{pubkey, system_program};
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
use spl_associated_token_account::create_associated_token_account;
use update_data::MyAccount;

#[derive(Debug)]
pub struct MyAccount2 {
    pub data_test: u64,
}

#[derive(Parser, Debug)]
pub struct Opts {
    #[clap(long)]
    pub token_address: Pubkey,
    #[clap(long)]
    pub bridge_contract_pid: Pubkey,
    #[clap(long)]
    pub receiver: Pubkey,
}

const MPL_PROGRAM_ID: &'static str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

//const BRIDGE_CONTRACT: &'static str = "F1eqWRT9CUruLk9n4mX4fCYKDqSde9yLtveRaywx6vwn";
//const TOKEN_ADDRESS: &'static str = "7YYNXbfwd5i5scpez18fTkEh2MRHJXoMHPffnWNcpFYf";
const SENDER: &'static str = "9hUYW9s2c98GfjZb6JvW62BYEt3ryxGmeMBkhgSqmZtW";
const SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID: &'static str = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";


// This example assumes a local validator is running with the programs
// deployed at the addresses given by the CLI args.
fn main() -> Result<()> {
    println!("Starting test...");
    let opts = Opts::parse();

    // Wallet and cluster params.
    let payer = read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json"))
        .expect("Example requires a keypair file");
    let url = Cluster::Custom(
        "https://api.devnet.solana.com".to_string(),
        "wss://api.devnet.solana.com/".to_string(),
    );

    // Client.
    let client = Client::new_with_options(url, Rc::new(payer), CommitmentConfig::processed());


    let pda_key = Pubkey::from_str("2hDqqrYVUkfuZi7iX46fRXN9p9hbehhNP1mcaZ6YcaSd").unwrap();
    let pid = Pubkey::from_str("2hvDgaUaPXetC969HmEtiMdw2kHWAVwZnXciLzFJ8wj6").unwrap();

    let program = client.program(pid);

    let counter_account: MyAccount = program.account(pda_key)?;

    println!("counter_account {:?}",counter_account);
    Ok(())
}