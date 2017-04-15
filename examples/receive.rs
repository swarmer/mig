extern crate mig;

#[macro_use]
extern crate log;
extern crate env_logger;

use mig::quic::threaded::{QuicListener};


fn main() {
    env_logger::init().unwrap();

    let args = ::std::env::args;
    if args().len() != 2 || args().nth(1) == Some("--help".to_string()) {
        println!("Usage: receive serverip:port > file");
        return;
    }

    let address = args().nth(1).unwrap();

    let listener = match QuicListener::bind(&*address) {
        Ok(listener) => {
            listener
        },
        Err(e) => {
            error!("Cannot bind to the address: {}", e);
            return;
        },
    };

    info!("Server on {}: listening for one connection...", address);

    let connection = match listener.accept() {
        Ok(connection) => {
            connection
        },
        Err(e) => {
            error!("Cannot accept a connection: {}", e);
            return;
        },
    };
    info!("Got a connection");

    let mut stream = connection.get_stream(2);

    info!("Got stream 2. Copying from the stream to stdout.");

    std::io::copy(&mut stream, &mut std::io::stdout()).unwrap();

    info!("Finished receiving.");
}

