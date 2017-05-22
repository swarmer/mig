#[macro_use]
extern crate log;
extern crate env_logger;
extern crate tun;

extern crate mig;

use std::str;
use std::io::{Write, Read};

use mig::quic::threaded::{QuicConnection,QuicListener};

use tun::{Device as TunDevice};

#[derive(Copy,Clone)]
enum Mode { Listen, Connect, }

fn get_connection(mode: Mode, address: &str) -> QuicConnection {
    match mode {
        Mode::Connect => {
            info!("Establishing connection...");
            let connection = match QuicConnection::new(address) {
                Ok(connection) => {
                    connection
                },
                Err(e) => {
                    error!("Cannot create a connection: {}", e);
                    ::std::process::exit(1)
                },
            };
        
            info!("Running client connected to {}", address);
            connection
        }
        Mode::Listen => {
            let listener = match QuicListener::bind(&*address) {
                Ok(listener) => {
                    listener
                },
                Err(e) => {
                    error!("Cannot bind to the address: {}", e);
                    panic!();
                },
            };
            info!("Server on {}: listening for connections...", address);
            let connection = match listener.accept() {
                Ok(connection) => {
                    connection
                },
                Err(e) => {
                    error!("Cannot accept a connection: {}", e);
                    panic!();
                },
            };
            info!("Got a connection");
            connection
        }
    }
}

fn main() {
    env_logger::init().unwrap();

    let args = ::std::env::args;
    if args().len() != 4 || args().nth(1) == Some("--help".to_string()) {
        println!("Usage: migtun <ifname> {{listen|connect}} <serverip>:<port>");
        return;
    }

    let tunname = args().nth(1).unwrap();
    let mode = match &*args().nth(2).unwrap() {
        "listen" => Mode::Listen,
        "connect" => Mode::Connect,
        &_ => panic!("Mode should be only listen or connect")
    };
    let address = args().nth(3).unwrap();
    info!("Creating tun device...");
    
    let mut dev = tun::create(tunname).unwrap();
    
    let connection = get_connection(mode, &*address);
 
    info!("Set the IP address of {} youself", dev.name());
    match mode {
        Mode::Connect => println!("ifconfig {} 192.168.123.2", dev.name()),
        Mode::Listen  => println!("ifconfig {} 192.168.123.1", dev.name()),
    }
    
    
    match mode { 
        Mode::Connect => {
            info!("Sending a byte");
            let b=[0;1];
            let mut stream = connection.get_stream(1);
            stream.write_all(&b).unwrap();
            
        }
        Mode::Listen  => {
            info!("Receiving a byte");
            let mut b=[0;1];
            let mut stream = connection.get_stream(1);
            stream.read_exact(&mut b).unwrap();
        }
    }
    
    
    info!("Serving");
    
    /*
    // Caution! Ugly dirty anti-rustic hack ahead
    let connection_copy = connection.clone();
    let dev_force_clone_and_send : (usize,usize);
    {
        let dev_ref : &mut TunDevice = &mut dev;
        dev_force_clone_and_send = unsafe { ::std::mem::transmute(dev_ref) };
    }
    
    ::std::thread::spawn(move || {
        let mut stream2 = connection_copy.get_stream(2);
        let dev2 : &mut TunDevice = unsafe {::std::mem::transmute(dev_force_clone_and_send)};
        
        std::io::copy(&mut stream2, dev2).unwrap();
    });*/
    
    let mut stream = connection.get_stream(2);
    match mode {
        Mode::Listen  => std::io::copy(&mut dev, &mut stream).unwrap(),
        Mode::Connect => std::io::copy(&mut stream, &mut dev).unwrap(),
    };
}
