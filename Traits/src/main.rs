fn main() {
    println!("Himanshu Rust adventure :)!");
    let items = vec![
        "Come to Linkedin".to_string(),
        "Come to Twitter".to_string(),
    ];
    print_all(items);
}

trait Printable {
    fn print(&self);
}

impl Printable for String {
    fn print(&self) {
        println!("{}", self);
    }
}

fn print_all<T: Printable>(items: Vec<T>) {
    for item in items {
        item.print();
    }
}
