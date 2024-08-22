use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    // let config = parse_arguments(&args);
    // let config = Config::new(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Arguments: {:?}", config.value);
    println!("File Path :{:?}", config.file_path);
    // will check if returns an error this will catch in if statement
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error: {e}");
        process::exit(1);
    }
}
