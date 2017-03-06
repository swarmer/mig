use std::collections::HashMap;
use std::fmt;
use std::net;
use std::net::ToSocketAddrs;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

use quic::engine::QuicEngine;
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
        unimplemented!()
    }
}
