use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // let config = parse_arguments(&args);
    let config = Config::new(&args);
    println!("Arguments: {:?}", config.value);
    println!("File Path :{:?}", config.file_path);

    let contents =
        fs::read_to_string(config.file_path).expect("should have been able to read the file");

    println!("with text:\n {:?}", contents);
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

impl Config {
    fn new(args: &[String]) -> Config {
        let value: &String = &args[1];
        let file_path: &String = &args[2];

        Config {
            value: value.to_string(),
            file_path: file_path.to_string(),
        }
    }
}
