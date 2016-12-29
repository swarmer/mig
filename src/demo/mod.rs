use env_logger;


pub fn mig_demo() -> i32 {
    env_logger::init().unwrap();

    debug!("mig demo started");

    0
}
