extern crate mig;

use std::process;

use mig::VERSION;


fn print_version() -> i32 {
    println!("{}", VERSION);
    0
}


fn main() {
    process::exit(print_version());
}
