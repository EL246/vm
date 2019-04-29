pub struct Parser {
    original_content: String,
    new_lines: Vec<String>,
}

impl Parser {
    pub fn new(content: &str) {
        parse_lines(content);
    }
}

pub fn parse_lines(content: &str) {
    let non_empty_lines: Vec<&str> = content.lines().
        filter(|line| !line.is_empty())
        .collect();

    for line in non_empty_lines {
        parse_line(line);
    }
}

pub fn parse_line(line: &str) {
//    remove comments:
//    let commented_segment = line.find("//").unwrap_or(line.len());
//    TODO: make this cleaner
    let split_line: Vec<&str> = line.splitn(2, "//").collect();
    let clean_line = split_line[0];
//    identify type of command
    if !clean_line.is_empty() {
        let command = get_command(&clean_line);
        print!("{:#?}", command);
    }
//    create command and add to vector that will be passed to CodeWriter later
}

//TODO: make this cleaner
//TODO: check for input errors?
fn get_command(line: &str) -> CommandType {
    if line.contains("push") {
        return parse_push(line);
    } else if line.contains("pop") {
        return parse_pop(line);
    }
//TODO: check for arithmetic operation explicitly later
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

#[derive(Debug)]
enum CommandType<'a> {
    Arithmetic { operation: String },
    Push { var_type: &'a str, var: &'a str },
    Pop { var_type: &'a str, var: &'a str },
}
