use super::Command;
use super::CommandType;

pub struct CodeWriter<'a> {
    lines_to_write: Vec<Command<'a>>,
    config: Config<'a>,
}

pub struct Config<'a> {
    stack_pointer: u32,
    local_segment_pointer: u32,
    arg_pointer: u32,
    this_pointer: u32,
    that_pointer: u32,
    static_filename: &'a str,
}

impl<'a> CodeWriter<'a> {
    pub fn new(lines_to_write: Vec<Command<'a>>, filename: &'a str) -> CodeWriter<'a> {
        let config = Config {
            stack_pointer: 0,
            local_segment_pointer: 1,
            arg_pointer: 2,
            this_pointer: 3,
            that_pointer: 4,
            static_filename: filename,
        };
        CodeWriter { lines_to_write, config }
    }

    pub fn handle(&self) {
        let mut result: Vec<String> = Vec::new();
        for command in &self.lines_to_write {
            result.push(command.orig_command_commented());
            let new_commands = self.get_command_lines(command);
        }
    }

    fn get_command_lines(&self, command: &Command) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let command_type = &command.command_type;
        match command_type {
            CommandType::Arithmetic { operation } => {
                return result;
            }
            CommandType::Push { var_type, var } => {
                return result
            }
            CommandType::Pop { var_type, var } => {
                return result;
            }
            _ => return result
        }
        result
    }
}

fn create_pop_command() {
    // pop static 17
    // ram[static+17] = *SP
    // SP--
}