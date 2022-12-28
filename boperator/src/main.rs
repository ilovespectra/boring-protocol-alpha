extern crate solana_sdk;

use solana_sdk::{
    account::{Account, KeyedAccount},
    pubkey::Pubkey,
    system_instruction,
};
use std::fs;
use std::io::Write;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    // Connect to the central server
    let mut stream = TcpStream::connect("central_server_address:port").unwrap();

    // Get the Boperator NFT ID
    let boperator_nft_id = Pubkey::new_rand();

    // Get the wallet address of the Solana wallet associated with the node
    let wallet_address = Pubkey::new_rand();

    // Check if the Boperator NFT is present in the wallet
    let mut wallet_account = Account::new(0, 0, &system_instruction::create_account::id(), &[]);
    let mut keyed_wallet_account = KeyedAccount::new(&wallet_address, false, &mut wallet_account);
    if !keyed_wallet_account.account.is_signer(&boperator_nft_id) {
        println!("Boperator NFT not found in wallet!");
        return;
    }

    // Start a thread to periodically send data usage updates to the central server
    thread::spawn(move || {
        loop {
            // Read the current data usage from the VPN interface
            let data_usage_str = fs::read_to_string("/sys/class/net/vpn0/statistics/tx_bytes").unwrap();
            let data_usage: u64 = data_usage_str.trim().parse().unwrap();

            // Send the data usage update to the central server
            writeln!(stream, "{}", data_usage).unwrap();

            // Sleep for a minute before sending the next update
            thread::sleep(Duration::from_secs(60));
        }
    });
}
