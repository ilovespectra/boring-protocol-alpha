use std::io::{self, Read};
use std::net::TcpListener;

fn main() {
    // Bind to a port on the local machine
    let listener = TcpListener::bind("0.0.0.0:12345").unwrap();

    // Accept incoming connections and process them in separate threads
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(move || {
            // Read data usage updates from the client
            let mut data_usage = String::new();
            stream.read_to_string(&mut data_usage).unwrap();

            // Process the data usage update
            process_data_usage_update(data_usage);
        });
    }
}

fn process_data_usage_update(data_usage: String) {
    // Process the data usage update (e.g., store it in a database, send it to a dashboard, etc.)
}
