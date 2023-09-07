#[derive(Debug)]
pub struct Config {
    pub input_file: Option<String>,
    pub output_dir: String,
    pub method: InputMethods,
}

#[derive(Debug)]
pub enum InputMethods {
    File,
    Spotify,
}

impl Config {
    pub fn build(mut args: std::env::Args) -> Result<Self, String> {
        if args.len() < 3 {
            return Err(String::from("Not Enough Args"));
        }
        args.next();

        let input_method_str = match args.next() {
            Some(mehod_str) => mehod_str,
            None => {
                eprintln!("Invalid Input Method Selected");
                eprintln!("1st argument -> File | Spotify");
                return Err("Input Method missing in args".to_string());
            }
        };

        let input_method = match &*input_method_str {
            "file" => InputMethods::File,
            "spotify" => InputMethods::Spotify,
            _ => {
                eprintln!("Invalid Input Method Selected");
                eprintln!("1st argument -> File | Spotify");
                return Err("Invalid Input Method".to_string());
            }
        };

        let input_file = if let InputMethods::File = input_method {
            match args.next() {
                Some(name) => Some(name),
                None => return Err(String::from("Input file missing in args")),
            }
        } else {
            None
        };

        let output_dir = match args.next() {
            Some(name) => name,
            None => return Err(String::from("Output dir missing in args")),
        };

        Ok(Config {
            input_file,
            output_dir,
            method: input_method,
        })
    }
}
