use std::env;
use std::process;

use vm::File;

fn main() {
    let file = File::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing filename");
        process::exit(1);
    });


}
