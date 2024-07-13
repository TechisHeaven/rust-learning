use std::error::Error;
use std::fs;

pub struct Config {
    pub value: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enought arguments");
        }
        let value = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { value, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("with text:\n {:?}", contents);

    Ok(())
}
