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
    // remove comments
//    let commented_segment = line.find("//").unwrap_or(line.len());
    let split_line: Vec<&str> = line.splitn(2,"//").collect();
    let clean_line = split_line[0];
    //identify type of command
//    get_command(clean_line);
    //create command and add to vector that will be passed to CodeWriter later
}

enum CommandType {
    Arithmetic {operation: String},
    Push{var_type: String, var: u32},
    Pop{var_type: String, var: u32},
}