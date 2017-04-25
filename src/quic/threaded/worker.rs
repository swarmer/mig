use std::collections::HashMap;
use std::io;
use std::net;
use std::net::{SocketAddr, ToSocketAddrs};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time;

use quic::engine::QuicEngine;
use quic::engine::udp_packet::{IncomingUdpPacket, OutgoingUdpPacket};
use quic::errors::{Error, Result};
use super::handle::{Handle, HandleGenerator};
use super::timer::ThreadedTimer;


#[derive(Debug, Default)]
struct WorkerConnection {
    connection_id: u64,
    data_available: Arc<Condvar>,
}


#[derive(Debug, Default)]
struct WorkerState {
    // checks
    started: bool,

    // engine
    engine: QuicEngine<ThreadedTimer>,

    // connections
    handle_generator: HandleGenerator,
    connection_map: HashMap<Handle, WorkerConnection>,

    connections_available: Arc<Condvar>,
}

impl WorkerState {
    fn handle_incoming_packet(&mut self, packet: IncomingUdpPacket) {
        self.engine.handle_incoming_packet(packet);
    }

    fn get_event_timeout(&self) -> time::Duration {
        self.engine.timer_ref().time_until_next_event()
            .unwrap_or_else(|| {
                trace!("No events pending, using default timeout");
                time::Duration::from_millis(50)
            })
    }

    fn signal_data_available(&self) {
        for connection in self.connection_map.values() {
            if self.engine.any_data_available(connection.connection_id) {
                connection.data_available.notify_all();
            }
        }
    }
}


#[derive(Debug)]
pub struct Worker {
    state: Mutex<WorkerState>,
    udp_socket: net::UdpSocket,
}

impl Worker {
    pub fn new<A: ToSocketAddrs>(addr: A, accept_connections: bool) -> Result<Arc<Worker>> {
        let udp_socket = net::UdpSocket::bind(addr)?;
        let worker_ref = Arc::new(
            Worker {
                state: Mutex::new(WorkerState {
                    started: false,
                    engine: QuicEngine::new(ThreadedTimer::new(), accept_connections),
                    handle_generator: HandleGenerator::new(),
                    connection_map: HashMap::new(),
                    connections_available: Arc::new(Condvar::new()),
                }),
                udp_socket: udp_socket,
            }
        );
        Self::spawn_thread(worker_ref.clone());
        Ok(worker_ref)
    }

    pub fn new_connection(&self, addr: SocketAddr) -> Result<Handle> {
        let mut state = self.state.lock().unwrap();

        let id = state.engine.initiate_connection(addr);

        let connection = WorkerConnection {
            connection_id: id,
            data_available: Arc::new(Condvar::new()),
        };
        let handle = state.handle_generator.generate();
        state.connection_map.insert(handle, connection);

        Ok(handle)
    }

    pub fn read(&self, handle: Handle, stream_id: u32, buf: &mut [u8]) -> Result<usize> {
        let (connection_id, data_available) = {
            let state = self.state.lock().unwrap();

            let connection = {
                state.connection_map.get(&handle)
                .ok_or(Error::InvalidHandle)?
            };

            (connection.connection_id, connection.data_available.clone())
        };

        {
            let mut state = self.state.lock().unwrap();

            while !state.engine.data_available(connection_id, stream_id) {
                state = data_available.wait(state).unwrap();
            }

            state.engine.read(connection_id, stream_id, buf)
        }
    }

    pub fn write(&self, handle: Handle, stream_id: u32, buf: &[u8]) -> Result<()> {
        let outgoing_packets = {
            let mut state = self.state.lock().unwrap();

            let connection_id = {
                state.connection_map.get(&handle)
                .ok_or(Error::InvalidHandle)?
                .connection_id
            };

            state.engine.write(connection_id, stream_id, buf)?;

            state.engine.pop_pending_packets()
        };

        self.send_packets(outgoing_packets);

        Ok(())
    }

    pub fn accept(&self) -> Result<Handle> {
        let connections_available = {
            let state = self.state.lock().unwrap();

            state.connections_available.clone()
        };

        {
            let mut state = self.state.lock().unwrap();

            trace!("Checking for new connections: {}", state.engine.have_connections());
            while !state.engine.have_connections() {
                state = connections_available.wait(state).unwrap();
            }
            trace!("Got a connection");
            let connection_id = state.engine.pop_new_connection();

            let connection = WorkerConnection {
                connection_id: connection_id,
                data_available: Arc::new(Condvar::new()),
            };
            let handle = state.handle_generator.generate();
            state.connection_map.insert(handle, connection);

            Ok(handle)
        }
    }

    pub fn finalize_outgoing_stream(&self, handle: Handle, stream_id: u32) -> Result<()> {
        let outgoing_packets = {
            let mut state = self.state.lock().unwrap();

            let connection_id = {
                state.connection_map.get(&handle)
                .ok_or(Error::InvalidHandle)?
                .connection_id
            };

            state.engine.finalize_outgoing_stream(connection_id, stream_id)?;

            state.engine.pop_pending_packets()
        };

        self.send_packets(outgoing_packets);

        Ok(())
    }

    fn send_packets(&self, outgoing_packets: Vec<OutgoingUdpPacket>) {
        for packet in outgoing_packets {
            debug!("Sending UDP packet (size: {})", packet.payload.len());
            if let Err(ref e) = self.udp_socket.send_to(&packet.payload[..], packet.destination_address) {
                error!("UDP send error: {:?}", e);
            }
        }
    }

    fn spawn_thread(worker_ref: Arc<Worker>) {
        {
            let mut state = worker_ref.state.lock().unwrap();
            if state.started {
                panic!("Worker thread already spawned");
            }
            state.started = true;
        }

        thread::spawn(move || {
            Self::run(worker_ref);
        });
    }

    fn run(worker_ref: Arc<Worker>) {
        debug!("Running QUIC worker");

        let udp_socket = &worker_ref.udp_socket;
        const UDP_BUF_SIZE: usize = 65535;
        let mut incoming_udp_buf = [0; UDP_BUF_SIZE];

        loop {
            // process scheduled events
            // get time until the next one
            // and get packets to send
            let (timeout, outgoing_packets) = {
                let mut state = worker_ref.state.lock().unwrap();

                let mut timeout = state.get_event_timeout();
                while timeout == time::Duration::from_secs(0) {
                    debug!("Processing due QUIC events");
                    state.engine.handle_due_events();
                    timeout = state.get_event_timeout();
                }

                (timeout, state.engine.pop_pending_packets())
            };

            // send pending packets
            worker_ref.send_packets(outgoing_packets);

            // receive a packet with a timeout
            trace!("Waiting to receive a UDP packet with timeout: {:?}", timeout);
            udp_socket.set_read_timeout(Some(timeout)).unwrap();
            let (packet_size, source_address) = match udp_socket.recv_from(&mut incoming_udp_buf) {
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    trace!("UDP receive timed out");
                    continue;
                },
                Err(ref e) => {
                    error!("UDP recv error: {:?}, kind: {:?}", e, e.kind());
                    continue;
                },
                Ok(result) => result,
            };
            if packet_size >= UDP_BUF_SIZE {
                error!("Dropping a jumbogram packet: not supported");
                continue;
            }

            debug!("Received UDP packet (size: {})", packet_size);
            let packet_data = &incoming_udp_buf[..packet_size];
            let packet = IncomingUdpPacket {
                source_address: source_address,
                payload: Vec::from(packet_data),
            };

            {
                let mut state = worker_ref.state.lock().unwrap();
                state.handle_incoming_packet(packet);

                trace!("Signaling connections_available: {}", state.engine.have_connections());
                if state.engine.have_connections() {
                    state.connections_available.notify_all();
                }

                state.signal_data_available();
            }
        }
    }
}
