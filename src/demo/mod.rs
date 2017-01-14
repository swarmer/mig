mod args;

use std;
use std::net::UdpSocket;

use env_logger;


fn run_server(address: String) -> i32 {
    let socket;
    match UdpSocket::bind(&*address) {
        Ok(s) => {
            socket = s;
        },
        Err(e) => {
            error!("Cannot bind to the address: {}", e);
            return 1;
        },
    }

    info!("Running server on {}", address);

    let mut inc_buf = [0; 256];
    let (amt, src) = socket.recv_from(&mut inc_buf).unwrap();

    let inc_string = std::str::from_utf8(&inc_buf).unwrap();
    info!("Got message: {}", inc_string);

    let out_buf = &inc_buf[..amt];
    socket.send_to(out_buf, &src).unwrap();

    0
}


fn run_client(address: String) -> i32 {
    let socket;
    match UdpSocket::bind(&*address) {
        Ok(s) => {
            socket = s;
        },
        Err(e) => {
            error!("Cannot bind to the address: {}", e);
            return 1;
        },
    }

    info!("Running client connected to {}", address);

    let message = "Hello";
    let out_buf = message.as_bytes();
    match socket.send_to(out_buf, &*address) {
        Ok(_) => {},
        Err(e) => {
            error!("{}", e);
            return 1;
        }
    }

    let mut inc_buf = [0; 256];
    let (amt, _) = socket.recv_from(&mut inc_buf).unwrap();
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
