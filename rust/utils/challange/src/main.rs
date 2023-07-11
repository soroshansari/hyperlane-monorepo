mod i_mailbox;
mod utils;

use clap::{Arg, Command};
use dotenv::dotenv;
use ethers::{
    prelude::*,
    providers::{Http, Provider},
    signers::LocalWallet,
    utils::hex,
};
use i_mailbox::IMailbox;
use std::{env, sync::Arc};

use crate::utils::{convert_address_to_bytes32, string_to_hex_bytes};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let matches = Command::new("Message CLI")
        .version("1.0")
        .author("Soroosh Ansarimehr <sorosh.ansari@gmail.com>")
        .about("A CLI tool for sending and querying messages")
        .subcommand(
            Command::new("send")
                .about("Send a message")
                .arg(Arg::new("mailbox_address").required(true).index(1))
                .arg(Arg::new("rpc_url").required(true).index(2))
                .arg(Arg::new("origin_chain_id").required(true).index(3))
                .arg(Arg::new("destination_chain_id").required(true).index(4))
                .arg(Arg::new("destination_address").required(true).index(5))
                .arg(Arg::new("message").required(true).index(6)),
        )
        .subcommand(
            Command::new("search")
                .about("Search for messages")
                .arg(Arg::new("chain").required(true).index(1))
                .arg(Arg::new("matching_list").required(true).index(2)),
        )
        .get_matches();

    // Handle subcommands and call corresponding functions
    match matches.subcommand() {
        Some(("send", send_matches)) => {
            // Convert mailbox address to Address
            let mailbox_address = send_matches
                .get_one::<String>("mailbox_address")
                .unwrap()
                .parse::<Address>()
                .unwrap();

            // Convert RPC URL to string
            let rpc_url = send_matches.get_one::<String>("rpc_url").unwrap();

            // Convert chain IDs to u32
            let origin_chain_id = send_matches
                .get_one::<String>("origin_chain_id")
                .unwrap()
                .parse::<u32>()
                .unwrap();

            // Convert destination chain id to u32
            let destination_chain_id = send_matches
                .get_one::<String>("destination_chain_id")
                .unwrap()
                .parse::<u32>()
                .unwrap();

            // Convert destination address to bytes32
            let destination_address: [u8; 32] = convert_address_to_bytes32(
                send_matches
                    .get_one::<String>("destination_address")
                    .unwrap(),
            );

            // Convert message to bytes
            let message_bytes =
                string_to_hex_bytes(send_matches.get_one::<String>("message").unwrap());
            let message = ethers::types::Bytes::from(message_bytes);

            // Call send_message function with the provided arguments
            send_message(
                mailbox_address,
                rpc_url,
                origin_chain_id,
                destination_chain_id,
                destination_address,
                message,
            )
            .await;
        }
        Some(("search", search_matches)) => {
            // Call search_messages function with the provided arguments
            search_messages(
                search_matches.get_one::<String>("chain").unwrap(),
                search_matches.get_one::<String>("matching_list").unwrap(),
                search_matches.get_one::<String>("rpc_url").unwrap(),
            );
        }
        _ => {
            // No subcommand provided or unknown subcommand
            println!("Invalid command");
        }
    }
}

async fn send_message(
    mailbox_address: H160,
    rpc_url: &str,
    origin_chain_id: u32,
    destination_chain_id: u32,
    destination_address: [u8; 32],
    message: Bytes,
) {
    // Connect to the Ethereum RPC provider
    let provider = Provider::<Http>::try_from(rpc_url).expect("Failed to connect to RPC endpoint");

    // Load the private key for signing the transaction
    let private_key = match env::var_os("PRIVATE_KEY") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$PRIVATE_KEY is not set"),
    };

    // Create a wallet from the private key
    let wallet: LocalWallet = private_key
        .parse::<LocalWallet>()
        .expect("Invalid private key")
        .with_chain_id(origin_chain_id);

    // Wrap the provider and wallet together to create a signer client
    let client = SignerMiddleware::new(provider.clone(), wallet.clone());

    // Create contract instance
    let contract = IMailbox::new(mailbox_address.clone(), Arc::new(client.clone()));

    // Call contract method
    let tx = contract
        .dispatch(destination_chain_id, destination_address, message)
        .await
        .expect("Failed to send transaction");

    // Display transaction receipt
    println!("Transaction id: {}", hex::encode(tx.to_vec()));
}

fn search_messages(chain: &str, matching_list: &str, rpc_url: &str) {
    // Implement search_messages functionality
}
