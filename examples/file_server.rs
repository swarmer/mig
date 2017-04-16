#[macro_use]
extern crate log;
extern crate env_logger;

extern crate mig;

use std::fs::File;
use std::io::{Read};
use std::str;

use mig::quic::threaded::{QuicListener};


fn main() {
    env_logger::init().unwrap();

    let args = ::std::env::args;
    if args().len() != 3 || args().nth(1) == Some("--help".to_string()) {
        println!("Usage: file_server <serverip>:<port> <filedir>");
        return;
    }

    let address = args().nth(1).unwrap();
    let filedir = args().nth(2).unwrap();

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

        let mut buf = vec![];
        let amt = stream.read_to_end(&mut buf).unwrap();
        let buf = &buf[..amt];

        let filename = str::from_utf8(&buf).unwrap();
        info!("Requested filename: {}", filename);

        let filepath = format!("{}/{}", filedir, filename);
        let mut requested_file = match File::open(filepath) {
            Ok(file) => file,
            Err(ref e) => {
                error!("Error opening file: {:?}", e);
                continue;
            }
        };

        std::io::copy(&mut requested_file, &mut stream).unwrap();
        info!("File sent");
    }
}

