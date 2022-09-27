pub mod nft;
pub mod utils;
extern crate core;

use std::cmp::min;
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
use clap::ErrorKind::NoEquals;
use mpl_token_metadata::state::Collection;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
#[cfg(feature = "serde-feature")]
use {
    serde::{Deserialize, Serialize},
    serde_with::{As, DisplayFromStr},
};
use token_middleware::token_middleware::nft_mint;
use crate::nft::mint_master_edition;
//import { TOKEN_PROGRAM_ID, createAssociatedTokenAccountInstruction, getAssociatedTokenAddress, createInitializeMintInstruction, MINT_SIZE } from '@solana/spl-token' // IGNORE THESE ERRORS IF ANY

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
//一键生成NFT的合约,待废弃
const NFT_MINT_CONTRACT: &'static str = "9HiRJw3dYo2MV9B1WrqFfoNjWRPS19mjVDCPqAxuMPfb";
const SENDER: &'static str = "9hUYW9s2c98GfjZb6JvW62BYEt3ryxGmeMBkhgSqmZtW";
const SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID: &'static str =
    "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";
const SYSTEM_PROGRAM_ID: &'static str = "11111111111111111111111111111111";
const SYSTEM_RENT_ID: &'static str = "SysvarRent111111111111111111111111111111111";
const MPL_TOKEN_METADATA_ACCOUNT: &'static str = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s";
const MEM_COLLECTION_MINT: &'static str = "8zKSXBACKpaKvgDCYdDwpJGTVDSBCtAgucJpmR7gAyx5";
const TOKEN_MIDDLEWARE: &'static str = "8ZjekeVj2PHuVmaTX2Ti7vv1tZy3THJ9fZY2JJxwMaQv";

pub trait Json {
    fn load<T: for<'a> Deserialize<'a>>(path: &str) -> Result<Vec<T>> {

        let mut file = File::open(path).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let data : Vec<T> = serde_json::from_str(&data)?;
        Ok(data)
    }
}

#[derive(Clone, Deserialize,Debug)]
pub struct ProjectInfo {
    pub project: String,
    pub collection_uri: String,
    pub token_uri: String,
    pub supply: u64,
}

impl Json for ProjectInfo{}




/*fn test_add_collection() -> Result<()>{
    let client = get_wallet("~/.config/solana/id.json".to_string());
    let mint_key = nft::mint(&client).unwrap();
    nft::add_collection(mint_key,Pubkey::from_str("2TDavXVuoknovjmVTyiUPaBdQGnTB7q4sJZK1yN7AGd5").unwrap())
}*/
pub fn get_wallet(keypair_path: String) -> Client{
    let payer = read_keypair_file(&*shellexpand::tilde(&keypair_path))
        .expect("Example requires a keypair file");
    //let payer = read_keypair_file(&*shellexpand::tilde("/Users/eddy/work/repo/solana/solana_dapp/my_wallet/3.json")).unwrap();
    let url = Cluster::Custom(
        "https://api.devnet.solana.com".to_string(),
        "wss://api.devnet.solana.com/".to_string(),
    );
    Client::new_with_options(url, Rc::new(payer), CommitmentConfig::processed())
}
fn main() -> Result<()> {
    let _opts = Opts::parse();
    let project_infos : Vec<ProjectInfo> = ProjectInfo::load("./upload_cids.json").unwrap();
    println!("{:#?}",project_infos);
    for project_info in project_infos {
        let collection_name = format!("{} collection",project_info.project);
        let collection_token = nft::mint_master_edition(&get_wallet("~/.config/solana/id.json".to_string()),
                                                        project_info.collection_uri.as_str(),
                                                        collection_name.as_str()).unwrap();
        let collection = Collection{
            verified: true,
            key: collection_token
        };
        for id in 0..project_info.supply {
            let token_name =  format!("{} #{}",project_info.project,id);
            let token_uri = format!("{}/{}.json",project_info.token_uri,id);
            let client = get_wallet("~/.config/solana/id.json".to_string());
            let collection_token = nft::mint(&client,
                                             token_uri.as_str(),
                                             token_name.as_str(),
                                             Some(collection.clone())
            ).unwrap();
        }
    }

    Ok(())
}
