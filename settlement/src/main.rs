use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::Command;

fn main() {
    // Get the Wireguard interface name
    let interface = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: data_usage INTERFACE");
        std::process::exit(1);
    });

    // Get the total number of bytes sent and received through the Wireguard interface
    let output = Command::new("/sbin/ifconfig")
        .arg(&interface)
        .output()
        .expect("failed to execute ifconfig");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut bytes_sent = 0;
    let mut bytes_received = 0;
    for line in stdout.lines() {
        if line.starts_with("          RX packets") {
            let fields: Vec<&str> = line.split_whitespace().collect();
            bytes_received += fields[5].parse::<u64>().unwrap();
        } else if line.starts_with("          TX packets") {
            let fields: Vec<&str> = line.split_whitespace().collect();
            bytes_sent += fields[5].parse::<u64>().unwrap();
        }
    }

    // Calculate the total data usage in GB
    let data_usage_gb = (bytes_sent + bytes_received) as f64 / 1e9;

    // Calculate the number of BOP tokens to award the Wireguard node based on the data usage
    let bop_tokens = (data_usage_gb * 0.01) as u64;

    // Send the BOP tokens to the Wireguard node's wallet address
    let wallet_address = "0x123456...";
    let send_command = format!("boring-protocol-cli send --to {} --value {}", wallet_address, bop_tokens);
    let output = Command::new("sh")
        .arg("-c")
        .arg(send_command)
        .output()
        .expect("failed to execute send command");
    if !output.status.success() {
        eprintln!("Error sending BOP tokens: {}", String::from_utf8_lossy(&output.stderr));
    }
}
