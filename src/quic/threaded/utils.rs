use std::io::{Error, ErrorKind, Result};
use std::net;


pub fn get_socket_addr<A: net::ToSocketAddrs>(addr: A) -> Result<net::SocketAddr> {
    addr.to_socket_addrs()?.next().ok_or(
        Error::new(ErrorKind::InvalidInput, "could not resolve to any addresses")
    )
}
