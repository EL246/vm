use super::Command;
use super::CommandType;

pub struct Parser<'a> {
    content: &'a str,
    pub result: Vec<Command<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new<>(content: &str) -> Parser {
        Parser {
            content,
            result: Vec::new(),
        }
    }

    pub fn handle(&mut self) {
        self.parse_lines();
    }

    fn parse_lines(&mut self) {
//        let mut commands = Vec::new();

        let non_empty_lines: Vec<&str> = self.content.lines().
            filter(|line| !line.is_empty())
            .collect();

        for line in non_empty_lines {
            if let Some(command) = parse_line(line) {
                self.result.push(command);
            }
        }
//        commands
    }
}


fn parse_line(line: &str) -> Option<Command, > {
//    let commented_segment = line.find("//").unwrap_or(line.len());
//    TODO: make this cleaner

//    remove comments
    let split_line: Vec<&str> = line.splitn(2, "//").collect();
    let clean_line = split_line[0];

//  identify type of command
    if !clean_line.is_empty() {
        let command_type = get_command(&clean_line);
        print!("{:#?}", command_type);

//      create command and add to vector that will be passed to CodeWriter later
        let new_command = Command { command_type, command: line };
        return Some(new_command);
    }

    None
}

// TODO: make this cleaner
// TODO: check for input errors?
fn get_command(line: &str) -> CommandType {
    if line.contains("push") {
        return parse_push(line);
    } else if line.contains("pop") {
        return parse_pop(line);
    }
// TODO: check for arithmetic operation explicitly
    let operation = line.replace("\\s", "");
    CommandType::Arithmetic { operation }
}

fn parse_operations(line: &str) -> (Option<&str, >, Option<&str, >) {
    let mut iter = line.split_whitespace();
    iter.next();
    let var_type = iter.next();
    let var = iter.next();
    (var_type, var)
}

//TODO: refactor parse_pop and parse_push into one method
fn parse_pop(line: &str) -> CommandType {
    if let (Some(var_type), Some(var)) = parse_operations(line) {
        return CommandType::Pop { var_type, var };
    }
    panic!("could not parse line")
}

fn parse_push(line: &str) -> CommandType {
    if let (Some(var_type), Some(var)) = parse_operations(line) {
        return CommandType::Push { var_type, var };
    }
    panic!("could not parse line")
}