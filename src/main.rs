use std::env;
use std::process;

use vm::File;

fn main() {
    let file = File::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing filename");
        process::exit(1);
    });

    if let Err(e) = vm::run(file) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
