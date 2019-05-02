use super::Command;
use super::CommandType;
use std::collections::HashMap;

pub struct CodeWriter<'a> {
    lines_to_write: Vec<Command<'a>>,
    config: Config<'a>,
    result: Vec<&'a str>,
}

pub struct Config<'a> {
    static_filename: &'a str,
    register_pointers: HashMap<&'a str, u32>,
}

impl<'a> CodeWriter<'a> {
    pub fn new(lines_to_write: Vec<Command<'a>>, filename: &'a str) -> CodeWriter<'a> {
        let mut register_pointers = HashMap::new();
        register_pointers.insert("SP", 0);
        register_pointers.insert("local", 1);
        register_pointers.insert("argument", 2);
        register_pointers.insert("this", 3);
        register_pointers.insert("that", 4);

        let config = Config {
            static_filename: filename,
            register_pointers,
        };
        let result = Vec::new();
        CodeWriter { lines_to_write, config, result }
    }

    pub fn handle(&mut self) {
//        let mut result: Vec<&str> = Vec::new();
        for command in &self.lines_to_write {
//            let mut vec: Vec<&str> = Vec::new();
            let mut new_commands = &mut get_command_lines(&command, &self.config);
            self.result.append(new_commands);
        }
    }
}


fn get_command_lines<'b>(command: &Command<'b>, config: &Config<'b>) -> Vec<&'b str> {
    let mut result: Vec<&str> = Vec::new();
    result.push(&command.orig_command_commented());

    let command_type = &command.command_type;
    match command_type {
        CommandType::Arithmetic { operation } => {}
        CommandType::Push { var_type, var } => {}
        CommandType::Pop { var_type, var } => {
            create_pop_command(&config, var_type, var, &mut result);
        }
        _ => ()
    }
    result
}

fn create_pop_command<'a>(config: &Config, var_type: &str, var: &str, result: &'a mut Vec<&'a str>) -> &'a mut Vec<&'a str> {
    // pop local 17
    // ram[local+17] = *SP
    // SP--
//        let mut result: Vec<&str> = Vec::new();

    append_get_register_val(config, var, var_type, result);

//        store in temp location (R13??)
    result.push("@R13");
    result.push("M=D");

//        get SP
//        TODO: introduce error handling for None() case

    let sp_str = format!("@{}", get_sp(config));
    result.push(&sp_str[..]);


    result.push("A=M");
    result.push("D=M");
    result.push("@R13");
    result.push("A=M");
    result.push("M=D");

//        SP--
//        TODO: change from Vec<String> to Vec<&str>
//        TODO: create separate method to get SP
    result.push(&sp_str);
    result.push("M=M-1");

    result
}

fn append_get_register_val(config: &Config, var: &str, var_type: &str, result: &mut Vec<&str>) {
    let i = var;
//        TODO: check for null...
    if let Some(index) = config.register_pointers.get(var_type) {

//        if index is of type arg/local/this/that:
//       @var_type
        let index_string = format!("@{}", index);
        result.push(&index_string);
//        D=M
        result.push("D=M");
//        @i
        let s = format!("@{}", i);
        result.push(&s);
//        D=D+A
        result.push("D=D+A");
    }
//        result
}


fn get_sp<'a>(config: &'a Config) -> &'a u32 {
    if let Some(SP) = config.register_pointers.get("SP") {
        return SP;
    }
    panic!("SP is not defined")
}