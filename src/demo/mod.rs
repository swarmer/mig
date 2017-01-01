mod args;

use env_logger;


pub fn mig_demo() -> i32 {
    env_logger::init().unwrap();

    match args::parse_command() {
        Ok(args::MigCommand::Version) => {
            println!("{}", ::VERSION);
            0
        },
        Ok(args::MigCommand::Server { address }) => {
            info!("Running server on {}", address);
            0
        },
        Ok(args::MigCommand::Client { address }) => {
            info!("Running client connected to {}", address);
            0
        },
        Err(err) => {
            println!("{}", &err.message);
            err.exit_code
        },
    }
}
