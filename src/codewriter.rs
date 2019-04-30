pub struct CodeWriter<'a> {
    lines_to_write: Vec<Command<'a>>,
}

impl CodeWriter {
    pub fn new(lines_to_write: Vec<Command>) -> CodeWriter {
        CodeWriter{lines_to_write}
    }
}