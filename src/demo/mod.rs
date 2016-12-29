mod args;

use env_logger;


pub fn mig_demo() -> i32 {
    env_logger::init().unwrap();

    match args::parse_command() {
        args::MigCommand::Version => {
            println!("{}", ::VERSION);
            0
        },
        args::MigCommand::Server { host, port } => {
            debug!("Running server on {}:{}", host, port);
            0
        },
        args::MigCommand::Client { host, port } => {
            debug!("Running client connected to {}:{}", host, port);
            0
        },
    }
}
