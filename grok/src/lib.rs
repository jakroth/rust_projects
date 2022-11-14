use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        println!("Searching for {}", query);
        println!("In file {}", file_path);

        Ok(Self { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> { //dyn = dynamic
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}
