use std::fs;
use std::error::Error;

mod parser;
mod codewriter;

pub fn run(file: File) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&file.filename)?;

    let new_lines = parse_lines(&contents);
    write_lines(new_lines, &file.filename);
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

fn parse_lines(content: &String) -> Vec<Command> {
    parser::Parser::new(content).handle()
}

fn write_lines(lines: Vec<Command>, filename: &str) {
    codewriter::CodeWriter::new(lines, filename).handle();
}

#[derive(Debug)]
pub enum CommandType<'a> {
    Arithmetic { operation: String },
    Push { var_type: &'a str, var: &'a str },
    Pop { var_type: &'a str, var: &'a str },
}

//impl Arithmetic {
//
//}

//impl CommandType::Pop {
//    fn create_commands() -> Vec<String> {
        // pop static 17
        // ram[static+17] = *SP
        // SP--
//    }
//}

//impl CommandType::Push {}

pub struct Command<'a> {
    command_type: CommandType<'a>,
    command: &'a str,
}

impl<'a> Command<'a> {
    pub fn orig_command_commented(&self) -> String {
        String::from("// ") + self.command
    }
}