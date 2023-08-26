pub struct Config {
    pub input_file: String,
    pub output_dir: String,
}

impl Config {
    pub fn build(mut args: std::env::Args) -> Result<Self, String> {
        if args.len() < 3 {
            return Err(String::from("Not Enough Args"));
        }
        args.next();
        let input_file = match args.next() {
            Some(name) => name,
            None => return Err(String::from("Input file missing in args")),
        };
        let output_dir = match args.next() {
            Some(name) => name,
            None => return Err(String::from("Output dir missing in args")),
        };
        Ok(Config{input_file,output_dir})
    }
}
