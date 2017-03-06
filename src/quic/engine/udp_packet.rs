use std::net;


#[derive(Clone, Debug, PartialEq)]
pub struct IncomingUdpPacket {
    pub source_address: net::SocketAddr,
    pub payload: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OutgoingUdpPacket {
    pub destination_address: net::SocketAddr,
    pub payload: Vec<u8>,
}
