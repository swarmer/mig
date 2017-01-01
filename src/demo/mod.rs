mod args;

use env_logger;


fn run_server(address: String) -> i32 {
    info!("Running server on {}", address);
    0
}


fn run_client(address: String) -> i32 {
    info!("Running client connected to {}", address);
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
