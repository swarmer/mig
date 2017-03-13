//! # Threaded QUIC connections
//! A QUIC API based on a threaded connection handler
mod handle;
mod timer;
mod worker;

use std;
use std::io;
use std::net::ToSocketAddrs;
use std::sync::Arc;

use quic::errors::Result;


#[derive(Debug)]
pub struct QuicConnection {
    worker_ref: Arc<worker::Worker>,
    handle: handle::Handle,
}

impl QuicConnection {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<QuicConnection> {
        let worker_ref = worker::Worker::new("0.0.0.0", false)?;
        let handle = worker_ref.connect(addr)?;
        Ok(QuicConnection { worker_ref: worker_ref, handle: handle })
    }

    pub fn get_stream(&self, stream_id: u32) -> QuicStream {
        QuicStream { connection: self, stream_id: stream_id }
    }
}


#[derive(Debug)]
pub struct QuicStream<'a> {
    connection: &'a QuicConnection,
    stream_id: u32,
}

impl<'a> io::Read for QuicStream<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        unimplemented!()
    }
}

impl<'a> io::Write for QuicStream<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        unimplemented!()
    }

    fn flush(&mut self) -> io::Result<()> {
        unimplemented!()
    }
}


#[derive(Debug)]
pub struct QuicListener {
    worker_ref: Arc<worker::Worker>,
}

impl QuicListener {
    pub fn bind<A: ToSocketAddrs>(addr: A) -> Result<QuicListener> {
        let worker_ref = worker::Worker::new(addr, true)?;
        Ok(QuicListener { worker_ref: worker_ref })
    }

    pub fn accept(&self) -> Result<QuicConnection> {
        // TODO
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }

        unimplemented!()
    }
}
