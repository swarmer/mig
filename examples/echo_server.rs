#[macro_use]
extern crate log;
extern crate env_logger;

extern crate mig;

use std::io::{Read, Write};
use std::str;

use mig::quic::threaded::{QuicListener};


fn main() {
    env_logger::init().unwrap();

    let args = ::std::env::args;
    if args().len() != 2 || args().nth(1) == Some("--help".to_string()) {
        println!("Usage: echo_server <serverip>:<port>");
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

    info!("Server on {}: listening for connections...", address);

    loop {
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

        let mut buf = vec![0; 256];
        loop {
            let size = stream.read(&mut buf).unwrap();

            if size == 0 {
                break;
            }

            stream.write(&mut buf[..size]).unwrap();
        }
    }
}

