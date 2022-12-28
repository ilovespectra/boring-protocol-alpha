// Creates a connection to a godmother

use std::fs;
use std::io::Write;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    // Connect to the central server
    let mut stream = TcpStream::connect("central_server_address:port").unwrap();

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
