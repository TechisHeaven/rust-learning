use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let value = &args[1];
    let file_path = &args[2];
    println!("Arguments: {:?}", value);
    println!("File Path :{:?}", file_path);

    let contents = fs::read_to_string(file_path).expect("should have been able to read the file");

    println!("with text:\n {:?}", contents);
}
