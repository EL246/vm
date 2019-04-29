use std::fs;
use std::error::Error;

mod parser;

pub fn run(file: File) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&file.filename)?;

    parse_lines(&contents);
//    write_file(&file.filename);

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

fn parse_lines(content: &String) {
    parser::Parser::new(content);
}

//TODO: implement writing to a file
/*
pub fn write_file(filename: &str) {

}*/
