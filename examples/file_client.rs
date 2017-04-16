#[macro_use]
extern crate log;
extern crate env_logger;

extern crate mig;

use std::io::{Read, Write};
use std::str;

use mig::quic::threaded::{QuicConnection};


fn main() {
    env_logger::init().unwrap();

    let args = ::std::env::args;
    if args().len() != 3 || args().nth(1) == Some("--help".to_string()) {
        println!("Usage: file_client <serverip>:<port> <filename>");
        return;
    }

    let address = args().nth(1).unwrap();
    let filename = args().nth(2).unwrap();

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

    info!("Requesting the file");
    stream.write_all(filename.as_bytes()).unwrap();
    stream.finalize();

    let mut inc_buf = vec![];
    let amt = stream.read_to_end(&mut inc_buf).unwrap();
    let inc_buf = &inc_buf[..amt];
    let inc_string = str::from_utf8(&inc_buf).unwrap();
    println!("{}", inc_string);
}
