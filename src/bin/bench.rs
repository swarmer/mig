extern crate mig;

use std::process;

use mig::benchmarks;


fn run_benchmarks() -> i32 {
    benchmarks::run_all_benchmarks();
    0
}


fn main() {
    process::exit(run_benchmarks());
}
