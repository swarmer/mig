#[macro_use]
extern crate log;
extern crate env_logger;

extern crate mig;

use std::sync::{Arc};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::str;
use std::thread;

use mig::quic::threaded::QuicConnection;


fn main() {
    env_logger::init().unwrap();

    let args = ::std::env::args;
    if args().len() != 3 || args().nth(1) == Some("--help".to_string()) {
        println!("Usage: tunnel_client <serverip>:<port> <targetip>:<port>");
        return;
    }

    let address = args().nth(1).unwrap();
    let target = args().nth(2).unwrap();

    let listener = match TcpListener::bind(&*address) {
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
        let (mut connection, _) = match listener.accept() {
            Ok(connection) => {
                connection
            },
            Err(e) => {
                error!("Cannot accept a connection: {}", e);
                return;
            },
        };
        let mut connection_worker = connection.try_clone().unwrap();
        info!("Got a connection");

        let quic_own = Arc::new(QuicConnection::new(target.clone()).unwrap());
        let quic_writer = quic_own.clone();

        let mut stream_own = quic_own.get_stream(2);

        let target_writer = thread::spawn(move || {
            let mut quic_writer = quic_writer.get_stream(2);

            let mut buf = vec![0; 4096];
            loop {
                let size = connection_worker.read(&mut buf).unwrap();;

                if size == 0 {
                    break;
                }

                quic_writer.write(&mut buf[..size]).unwrap();
            }
        });

        let mut buf = vec![0; 4096];
        loop {
            let size = stream_own.read(&mut buf).unwrap();;

            if size == 0 {
                break;
            }

            connection.write(&mut buf[..size]).unwrap();
        }

        target_writer.join().unwrap();
    }
}

