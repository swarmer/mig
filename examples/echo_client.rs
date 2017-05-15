#[macro_use]
extern crate log;
extern crate env_logger;

extern crate mig;

use std::io::{Read, stdin, stdout, Write};
use std::str;

use mig::quic::threaded::{QuicConnection};


fn main() {
    env_logger::init().unwrap();

    let args = ::std::env::args;
    if args().len() != 2 || args().nth(1) == Some("--help".to_string()) {
        println!("Usage: echo_client <serverip>:<port>");
        return;
    }

    let address = args().nth(1).unwrap();

    info!("Establishing connection...");
    let connection = match QuicConnection::new(&*address) {
        Ok(connection) => {
            connection
        },
        Err(e) => {
            error!("Cannot create a connection: {}", e);
            return;
        },
    };

    info!("Running client connected to {}", address);

    let mut stream = connection.get_stream(2);

    let mut stdin = stdin();
    let mut stdout = stdout();
    let mut buf = vec![0; 256];
    loop {
        let size = stdin.read(&mut buf).unwrap();

        if size == 0 {
            break;
        }

        stream.write(&mut buf[..size]).unwrap();

        let size = stream.read(&mut buf).unwrap();

        assert!(size != 0);

        stdout.write_all(&mut buf[..size]).unwrap();
    }
}
