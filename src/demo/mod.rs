mod args;
mod benchmarks;

use std;
use std::io::{Read, Write};

use env_logger;

use quic::threaded::{QuicConnection, QuicListener};


fn run_server(address: String) -> i32 {
    let listener = match QuicListener::bind(&*address) {
        Ok(listener) => {
            listener
        },
        Err(e) => {
            error!("Cannot bind to the address: {}", e);
            return 1;
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
                return 1;
            },
        };
        info!("Got a connection");

        let mut stream = connection.get_stream(2);

        let mut buf = vec![];
        let amt = stream.read_to_end(&mut buf).unwrap();
        let buf = &buf[..amt];

        let inc_string = std::str::from_utf8(&buf).unwrap();
        info!("Got message: {}", inc_string);

        stream.write_all(buf).unwrap();
    }
}


fn run_client(address: String) -> i32 {
    let connection = match QuicConnection::new(&*address) {
        Ok(connection) => {
            connection
        },
        Err(e) => {
            error!("Cannot create a connection: {}", e);
            return 1;
        },
    };

    info!("Running client connected to {}", address);

    let mut stream = connection.get_stream(2);

    let message = std::iter::repeat("hello").take(1000).collect::<String>();
    let out_buf = message.as_bytes();
    stream.write_all(out_buf).unwrap();
    stream.finalize();

    let mut inc_buf = vec![];
    let amt = stream.read_to_end(&mut inc_buf).unwrap();
    let inc_buf = &inc_buf[..amt];
    let inc_string = std::str::from_utf8(&inc_buf).unwrap();
    info!("Got message: {}", inc_string);

    0
}


pub fn mig_demo() -> i32 {
    env_logger::init().unwrap();

    match args::parse_command() {
        Ok(args::MigCommand::Version) => {
            println!("{}", ::VERSION);
            0
        },
        Ok(args::MigCommand::Bench) => {
            benchmarks::run_all_benchmarks();
            0
        },
        Ok(args::MigCommand::Server { address }) => {
            run_server(address)
        },
        Ok(args::MigCommand::Client { address }) => {
            run_client(address)
        },
        Err(err) => {
            println!("{}", &err.message);
            err.exit_code
        },
    }
}
