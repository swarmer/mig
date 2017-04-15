extern crate mig;

#[macro_use]
extern crate log;
extern crate env_logger;

use mig::quic::threaded::{QuicConnection};


fn main() {
    env_logger::init().unwrap();

    let args = ::std::env::args;
    if args().len() != 2 || args().nth(1) == Some("--help".to_string()) {
        println!("Usage: upload clientip:port < file");
        return;
    }

    let address = args().nth(1).unwrap();

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

    info!("Got stream 2. Copying from the stream to stdout.");

    //std::io::copy(&mut stream, &mut std::io::stdout());
    std::io::copy(&mut std::io::stdin(), &mut stream).unwrap();

    info!("Finished uploading.");
}

