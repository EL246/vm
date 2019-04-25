use std::fs;
use std::error::Error;

pub fn run(file: File) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(file.filename)?;

    Ok(())
}

pub struct File {
    pub filename: String,
}

impl File {
    pub fn new(mut args: std::env::Args) -> Result<File, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a file name"),
        };

        Ok(File { filename })
    }
}