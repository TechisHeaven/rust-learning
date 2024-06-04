// fn main() {
//     todo_list();
//     define_enum();
// }

// fn define_enum (){
//         #[derive(Debug)]
//         enum IpAddrKind {
//             V4(u8 , u8 ,u8 , u8),
//             V6(String),
//         };

//         let home = IpAddrKind::V4(127 , 0, 0 ,1);

//         let loopback = IpAddrKind::V6(String::from("::1"));

//         println!("home address is {:?} and v6 loopback is {:?}", home , loopback);
// }

// fn todo_list(){
//     let action = std::env::args().nth(1).expect("Please specify an action");
//     let item = std::env::args().nth(2).expect("Please specify an item");

// println!("{:?}, {:?}", action, item);
// }

fn main() {
    let string = String::from("Hello world!");
    let result = find_first_index(string);

    match result {
        Some(index) => {
            println!("The Letter 's' Found at {} index", index);
        }
        None => {
            println!("The letter 's' not found at any of the index");
        }
    }
}

fn find_first_index(s: String) -> Option<i32> {
    for (index, character) in s.chars().enumerate() {
        if (character == 's') {
            return Some(index as i32);
        }
    }
    return None;
}
