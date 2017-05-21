#[macro_use]
extern crate log;
extern crate env_logger;

extern crate mig;

use std::sync::{Arc};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;
use std::thread;

use mig::quic::threaded::{QuicListener};


fn main() {
    env_logger::init().unwrap();

    let args = ::std::env::args;
    if args().len() != 3 || args().nth(1) == Some("--help".to_string()) {
        println!("Usage: tunnel_server <serverip>:<port> <targetip>:<port>");
        return;
    }

    let address = args().nth(1).unwrap();
    let target = args().nth(2).unwrap();

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
        let connection = Arc::new(connection);
        let connection_worker = connection.clone();
        info!("Got a connection");

        let mut stream_own = connection.get_stream(2);

        let mut target_stream_own = TcpStream::connect(target.clone()).unwrap();
        let mut target_stream_writer = target_stream_own.try_clone().unwrap();

        let target_writer = thread::spawn(move || {
            let mut stream_writer = connection_worker.get_stream(2);

            let mut buf = vec![0; 4096];
            loop {
                let size = stream_writer.read(&mut buf).unwrap();;

                if size == 0 {
                    break;
                }

                target_stream_writer.write(&mut buf[..size]).unwrap();
            }
        });

        let mut buf = vec![0; 4096];
        loop {
            let size = target_stream_own.read(&mut buf).unwrap();;

            if size == 0 {
                break;
            }

            stream_own.write(&mut buf[..size]).unwrap();
        }

        target_writer.join().unwrap();
    }
}

