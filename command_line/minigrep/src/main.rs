use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let value = &args[1];
    println!("Arguments: {:?}", value);
}
