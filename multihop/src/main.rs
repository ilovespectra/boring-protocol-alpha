// Double IP bounce

use std::process::Command;

fn main() {
    // Create a new Wireguard interface
    Command::new("wg-quick")
        .arg("up")
        .arg("wg0")
        .output()
        .expect("failed to create Wireguard interface");

    // Add the first IP hop
    Command::new("wg")
        .arg("set")
        .arg("wg0")
        .arg("peer")
        .arg("peer1_public_key")
        .arg("allowed-ips")
        .arg("10.0.1.0/24")
        .output()
        .expect("failed to add first IP hop");

    // Add the second IP hop
    Command::new("wg")
        .arg("set")
        .arg("wg0")
        .arg("peer")
        .arg("peer2_public_key")
        .arg("allowed-ips")
        .arg("10.0.2.0/24")
        .output()
        .expect("failed to add second IP hop");
}
