use std::cmp::min;
use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use anchor_client::solana_sdk::signature::read_keypair_file;
use anchor_client::solana_sdk::signature::{Keypair, Signer};
use anchor_client::solana_sdk::system_instruction;
use anchor_client::{Client, ClientError, Cluster, EventContext};
use mpl_token_metadata::instruction::{set_and_verify_collection,verify_collection};

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

//import { TOKEN_PROGRAM_ID, createAssociatedTokenAccountInstruction, getAssociatedTokenAddress, createInitializeMintInstruction, MINT_SIZE } from '@solana/spl-token' // IGNORE THESE ERRORS IF ANY

use spl_token::instruction::initialize_mint;
use nft_mint_and_verify::instruction as nft_instructions;
use nft_mint_and_verify::accounts as nft_accounts;


#[derive(Parser, Debug)]
pub struct Opts {
    #[clap(long)]
    pub token_address: Pubkey,
    #[clap(long)]
    pub bridge_contract_pid: Pubkey,
    #[clap(long)]
    pub receiver_wallet: Pubkey,
}

const SPL_PROGRAM_ID: &'static str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

//const BRIDGE_CONTRACT: &'static str = "F1eqWRT9CUruLk9n4mX4fCYKDqSde9yLtveRaywx6vwn";
//const TOKEN_ADDRESS: &'static str = "7YYNXbfwd5i5scpez18fTkEh2MRHJXoMHPffnWNcpFYf";
const SENDER: &'static str = "9hUYW9s2c98GfjZb6JvW62BYEt3ryxGmeMBkhgSqmZtW";
const SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID: &'static str = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";
const SYSTEM_PROGRAM_ID: &'static str = "11111111111111111111111111111111";
const SYSTEM_RENT_ID: &'static str = "SysvarRent111111111111111111111111111111111";
const MPL_TOKEN_METADATA_ACCOUNT: &'static str = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s";



pub fn find_metadata_pda(mint: &Pubkey) -> Pubkey {
    let (pda, _bump) = find_metadata_account(mint);

    pda
}

pub fn find_master_edition_pda(mint: &Pubkey) -> Pubkey {

    let (pda, _bump) = find_master_edition_account(mint);
    pda
}

pub fn get_acc(address: Pubkey) -> solana_sdk::account::Account{
    let rpc_client = RpcClient::new(
        "https://api.devnet.solana.com".to_string(),
    );
    rpc_client
        .get_account_with_commitment(&address, CommitmentConfig::processed()).unwrap()
        .value
        .ok_or(ClientError::AccountNotFound).unwrap()
}


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
    mint_nft(&client, opts)?;
    Ok(())
}

fn mint_nft(client: &Client, params: Opts) -> Result<()> {
    let program = client.program(params.bridge_contract_pid); //9HiRJw3dYo2MV9B1WrqFfoNjWRPS19mjVDCPqAxuMPfb
    let authority = program.payer();
    let to_wallet = program.payer();
    let payer = read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json")).expect("Example requires a keypair file");
    let nft_mint_key = Keypair::new();
    println!("nft mint key {}", nft_mint_key.pubkey().to_string());

    //当前记忆碎皮的集合的meta_account
    let memorise_mint_account = "6P64iPbit6iUbwMj55pXXEu7GxUaE9jPVqWCmomyqPph";
    let nft_token_account = get_associated_token_address(&to_wallet,&nft_mint_key.pubkey());
    let receiver_token_account = get_associated_token_address(&params.receiver_wallet, &nft_mint_key.pubkey());
    let metadata_address = find_metadata_pda(&nft_mint_key.pubkey());
    let edition_address = find_master_edition_pda(&nft_mint_key.pubkey());

    let account_init_build = program
        .request()
        .accounts(nft_accounts::AccountInit{
            mint: nft_mint_key.pubkey(),
            token_account: nft_token_account,
            mint_authority: payer.pubkey(),
            rent: Pubkey::from_str(SYSTEM_RENT_ID).unwrap(),
            system_program: Pubkey::from_str(SYSTEM_PROGRAM_ID).unwrap(),
            token_program: Pubkey::from_str(SPL_PROGRAM_ID).unwrap(),
            associated_token_program: Pubkey::from_str(SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID).unwrap(),
        })
        .args(nft_instructions::AccountInit);


    let mint_build = program
        .request()
        .accounts(nft_accounts::MintNFT{
            metadata: metadata_address, //
            master_edition: edition_address, //
            mint: nft_mint_key.pubkey(), //
            token_account: nft_token_account, //
            mint_authority: payer.pubkey(), //
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


    let verify_build = program
        .request()
        .accounts(nft_accounts::SetAndVerifyCollection{
            metadata_account: find_metadata_pda(&nft_mint_key.pubkey()),
            collection_authority: payer.pubkey(),
            payer: payer.pubkey(),
            update_authority: payer.pubkey(),
            collection_mint: Pubkey::from_str(memorise_mint_account).unwrap(),
            collection_metadata: find_metadata_pda(&Pubkey::from_str(memorise_mint_account).unwrap()),
            collection_master_edition: find_master_edition_pda(&Pubkey::from_str(memorise_mint_account).unwrap()),
            system_program: Pubkey::from_str(SYSTEM_PROGRAM_ID).unwrap(),
            rent:Pubkey::from_str(SYSTEM_RENT_ID).unwrap(),
            spl_token_metadata:Pubkey::from_str(MPL_TOKEN_METADATA_ACCOUNT).unwrap(),

            //for transfer
            mint_account: nft_mint_key.pubkey(),
            sender_token_account: nft_token_account,
            receiver_token_account: receiver_token_account,
            receiver_wallet: params.receiver_wallet,
            associated_token_program: Pubkey::from_str(SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID).unwrap(),
            spl_token_program: Pubkey::from_str(SPL_PROGRAM_ID).unwrap()

        })
        .args(nft_instructions::CollectionAdd);



    let call_res = program
        .request()
        .instruction(
            account_init_build.instructions()?.first().unwrap().to_owned()
        )
        .instruction(
            mint_build.instructions()?.first().unwrap().to_owned()
        )
        .instruction(
            verify_build.instructions()?.first().unwrap().to_owned()
        )
        .signer(&nft_mint_key)
        .signer(&payer)
        .send()?;
    println!("call res {}", call_res);

    Ok(())
}