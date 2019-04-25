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

        Ok(File{filename})
    }
}