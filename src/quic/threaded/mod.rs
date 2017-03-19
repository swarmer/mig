//! # Threaded QUIC connections
//! A QUIC API based on a threaded connection handler
mod handle;
mod timer;
mod utils;
mod worker;

use std;
use std::io;
use std::net::ToSocketAddrs;
use std::sync::Arc;

use quic::errors::Result;
use self::utils::get_socket_addr;


#[derive(Debug)]
pub struct QuicConnection {
    worker_ref: Arc<worker::Worker>,
    handle: handle::Handle,
}

impl QuicConnection {
    pub fn new<A: ToSocketAddrs>(addr: A) -> Result<QuicConnection> {
        let addr = get_socket_addr(addr)?;
        let worker_ref = worker::Worker::new("0.0.0.0:0", false)?;
        let handle = worker_ref.new_connection(addr)?;
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

impl<'a> QuicStream<'a> {
    pub fn finalize(&self) {
        let handle = self.connection.handle;
        let stream_id = self.stream_id;

        self.connection.worker_ref.finalize_outgoing_stream(handle, stream_id).unwrap();
    }
}

impl<'a> Drop for QuicStream<'a> {
    fn drop(&mut self) {
        self.finalize();
    }
}

impl<'a> io::Read for QuicStream<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let handle = self.connection.handle;
        let stream_id = self.stream_id;

        self.connection.worker_ref.read(handle, stream_id, buf)
            .map_err(|e| e.into())
    }
}

impl<'a> io::Write for QuicStream<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let handle = self.connection.handle;
        let stream_id = self.stream_id;

        self.connection.worker_ref.write(handle, stream_id, buf)
            .map_err::<io::Error, _>(|e| e.into())?;
        
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
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
        let handle = self.worker_ref.accept()?;

        Ok(QuicConnection { worker_ref: self.worker_ref.clone(), handle: handle })
    }
}
