fn main() {
  //  let s1 = String::from("Hello world");

   // let s2 = s1.clone();
    
   // println!("{} , {}", s1 , s2);

   // stack_only_copy();
   ownership_and_functions();
}

/*
fn stack_only_copy(){
    let x = 5;
    let y = x;

    println!("x = {} , y={}", x, y);
}
*/


fn ownership_and_functions(){
    let mut s = String::from("Hello String");
    let newString = &s;

    println!("borrowed with referance {}", newString);
    // s =  take_ownership(s);

    // or we can send data with refernce of that string 
   take_ownership(&mut s);

    println!("mutable takeback ownership string or with refernce value based on function &with this : {}", s);

    let n = 2;

    makes_copy(n);

    // println!("string={}, number={}", s, n);  //that will return error if re use s variable because of moved ownership


    let string2 = give_ownership();

    println!("{}", string2);

    reference_borrowing();
    // mutable_reference();
    multi_reference();

    let mut string = String::from("test string");
    let word = first_word(&string);
    let word = first_word(&string); // we can initialize function or variable mutliple time with returnable function which return first words

    println!("{}", word);
}




 fn give_ownership() -> String{
        let string = String::from("Hello there");

        string
    }


fn take_ownership(some_string: &mut String)-> &mut String{
    some_string.push_str("Changed at little bit");
      println!("{}",some_string);
      return some_string
}


fn makes_copy(some_interger: i32){
    println!("{}", some_interger);
}





fn reference_borrowing(){
    let string = String::from("Himanshu");

    let len = calculate_length(&string);


    println!("length of {} is {}", string, len);
}


// calculate length 

fn calculate_length(s: &String) -> usize {
    s.len()
}





// use mutable refernce borrowing

/*
fn mutable_reference(){
    let mut string = String::from("Himanshu");
    
    let changed_string = change(&mut string);

    println!("{} | {}" , string , changed_string);
}
*/


fn change(some_string: &mut String) -> &mut String {
    some_string.push_str(" verma");

    some_string
}



fn multi_reference(){
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}" , r1 , r2);

    let r3 = &mut s;

    println!("{}", r3);
}


fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for(i , &item) in bytes.iter().enumerate() {
        if(item == b' '){ // make changes here where to slice the string ' ' or any character
            return &s[0..i]; 
        }
    }
    &s[..]
}


