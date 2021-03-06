mod packet_decoding;
mod packet_encoding;

use std::time;


pub fn run_all_benchmarks() {
    println!("Running all benchmarks");

    packet_decoding::run_benchmark();
    packet_encoding::run_benchmark();
}


pub fn format_duration(duration: time::Duration) -> String {
    let secs = duration.as_secs();
    let ms = duration.subsec_nanos() / 1000000;

    format!("{}.{:03} secs", secs, ms)
}
