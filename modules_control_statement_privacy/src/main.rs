
pub mod pets;
pub mod garden;
pub mod front_of_house;

use pets::cats::Cat;
use garden::vegetables::Asparagus;
use	crate::front_of_house::eat_at_restaurant;

fn main(){

	eat_at_restaurant();

	let cleo = Cat {
		name : String::from("Cleo"),
		color: String::from("Gray"),
		age: 2,
	};


	let vegetable = Asparagus {
		family : String::from("Asparagaceae"),
		growth : 2,
	};


	println!("{:?}", cleo);
	println!("my cat name is {} and its age is {} and you know its beautiful color is {}", cleo.name , cleo.age , cleo.color);


	println!("my vegetable of family {} name is currenlty have growth hight of {} meter",vegetable.family , vegetable.growth );
}
