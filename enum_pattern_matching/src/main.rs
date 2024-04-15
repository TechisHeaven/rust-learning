fn main() {
    todo_list();
    define_enum();
}



fn define_enum (){
        #[derive(Debug)]
        enum IpAddrKind {
            V4(u8 , u8 ,u8 , u8), 
            V6(String),
        };


        let home = IpAddrKind::V4(127 , 0, 0 ,1);

        let loopback = IpAddrKind::V6(String::from("::1"));
            

        println!("home address is {:?} and v6 loopback is {:?}", home , loopback);
}


fn todo_list(){
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");

println!("{:?}, {:?}", action, item);
}