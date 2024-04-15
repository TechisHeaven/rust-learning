// // use std::u32;

// use std::io;

// fn main() {
//     // let mut x = 5;
//     // println!("The value of x is: {x}");
//     // x = 6;
//     // println!("The value of x is: {x}");

//     // let spaces = "    ";
//     // let spaces = spaces.len();

//     // print!("{}", spaces);

//     // let x = 28;
//     // let y: f32 = 28.1;
//     // // remainder
//     // let remainder = 43 % 5;
//     // println!("{} and other variable {}", y, remainder);
//     // let tup = (500, 6.4, 1);

//     // let (x, y, z) = tup;

//     // println!("The value of y is: {y}");
//     let a = [1, 2, 3, 4, 5];
//     let array_size: usize = a.len() - 1;
//     println!("Please enter an array index max value {array_size}.");

//     let mut index = String::new();

//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");

//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("Index entered was not a number");

//     let element = a[index];

//     println!("The value of the element at index {index} is: {element}");
// }

// fn main() {
//     print_labeled_measurement(1, 'h');
//     let y = {
//         let x = 3;
//         x + 1
//     };
//     println!("The value of y is: {y}");
// }

//fn print_labeled_measurement(value: i32, unit_label: char) {
//  println!("The measurement is: {value}{unit_label}");
//}

// fn main() {
//     //lable_loop_function();
//     //while_loop_function();
//     // loop_with_reverse();
// }

// fn lable_loop_function (){
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }
//     println!("End count = {count}");
// }

// fn while_loop_function(){
//      let mut number = 3;
//     while number != 0 {
//         println!("{number}!");

//         number -= 1;
//     }

//     println!("LIFTOFF!!!");
// }

// fn loop_with_reverse (){
//     for number in (1..4).rev() {
//         println!("{number}!");
//     }
//     println!("LIFTOFF!!!");


fn main() {
    let mut s = String::from("Hello");

    s.push_str(", World!");

    println!("{}", s);

    let text = String::from("Testing! message = ");

    let result = text;

    println!("{}", result);

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let (string, length) = LengthCalculator(s1);

    println!("Length of String {} is {}", string, length);
}

fn LengthCalculator(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
