fn main() {
    let string1 = String::from("hello");
    let string2 = String::from("world");

    let result = longest(string1.as_str(), string2.as_str());
    println!("largest string is {result}");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if (x.len() > y.len()) {
        x
    } else if (x.len() == y.len()) {
        x
    } else {
        y
    }
}
