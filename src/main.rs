use kps;
use std::process;

fn main() {
    if let Err(e) = kps::run() {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
