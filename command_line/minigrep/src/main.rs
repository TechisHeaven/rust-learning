use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    // let config = parse_arguments(&args);
    // let config = Config::new(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Arguments: {:?}", config.value);
    println!("File Path :{:?}", config.file_path);
    // will check if returns an error this will catch in if statement
    if let Err(e) = run(config) {
        println!("Application Error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("with text:\n {:?}", contents);

    Ok(())
}

struct Config {
    value: String,
    file_path: String,
}

// fn parse_arguments(args: &[String]) -> Config {
//     let value: &String = &args[1];
//     let file_path: &String = &args[2];

//     Config {
//         value: value.to_string(),
//         file_path: file_path.to_string(),
//     }
// }

// impl Config {
//     fn new(args: &[String]) -> Config {
//         if args.len() < 3 {
//             panic!("not enough arguments...")
//         }
//         let value: &String = &args[1];
//         let file_path: &String = &args[2];

//         Config {
//             value: value.to_string(),
//             file_path: file_path.to_string(),
//         }
//     }
// }

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enought arguments");
        }
        let value = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { value, file_path })
    }
}
