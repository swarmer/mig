use std::collections::HashMap;
use std::fmt;
use std::net;
use std::net::ToSocketAddrs;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time;

use quic::engine::QuicEngine;
use quic::engine::udp_packet::{IncomingUdpPacket, OutgoingUdpPacket};
use quic::errors::Result;
use super::handle::{Handle, HandleGenerator};
use super::timer::ThreadedTimer;


#[derive(Default)]
struct WorkerConnection {
    connection_id: u64,
    data_available: Condvar,
}

impl fmt::Debug for WorkerConnection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "WorkerConnection {{ connection_id: {}, data_available: Condvar }}",
            self.connection_id
        )
    }
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
}

impl WorkerState {
    fn handle_incoming_packet(&mut self, packet: IncomingUdpPacket) -> Vec<OutgoingUdpPacket> {
        // TODO
        unimplemented!()
    }

    fn get_event_timeout(&self) -> time::Duration {
        self.engine.timer_ref().time_until_next_event()
            .unwrap_or(time::Duration::from_millis(100))
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
                }),
                udp_socket: udp_socket,
            }
        );
        Self::spawn_thread(worker_ref.clone());
        Ok(worker_ref)
    }

    pub fn connect<A: ToSocketAddrs>(&self, addr: A) -> Result<Handle> {
        unimplemented!()
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
            let timeout = {
                let mut state = worker_ref.state.lock().unwrap();
                state.get_event_timeout()
            };
            udp_socket.set_read_timeout(Some(timeout)).unwrap();
            trace!("Waiting to receive a UDP packet with timeout: {:?}", timeout);

            let (packet_size, source_address) = match udp_socket.recv_from(&mut incoming_udp_buf) {
                Err(ref e) => {
                    error!("UDP recv error: {:?}", e);
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

            let outgoing_packets = {
                let mut state = worker_ref.state.lock().unwrap();
                state.handle_incoming_packet(packet);

                state.engine.pop_pending_packets()
            };

            for packet in outgoing_packets {
                if let Err(ref e) = udp_socket.send_to(&packet.payload[..], packet.destination_address) {
                    error!("UDP send error: {:?}", e);
                    continue;
                }
            }
        }
    }
}
