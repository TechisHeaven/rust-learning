pub mod hosting;


/*

// can directly define modules
pub mod hosting {
		pub fn add_to_waitlist(){
			println!("added to waitlist")
		}
		fn seat_at_table(){}

}
*/

pub fn eat_at_restaurant(){
	//absolute path 
	// crate::front_of_house::hosting::add_to_waitlist();

	//relative path
	hosting::add_to_waitlist();

}


// or use like this better way

