use std::collections::HashMap;


fn main() {
	Vectors();
	hashMaps();
}


fn Vectors(){
	let mut vector: Vec<i32> = Vec::new();
	let vector2 = vec![1, 2 , 3];

	vector.push(2);
	vector.push(3);
	vector.push(4);



	println!("mutable vectors {:?}" , vector);
}



fn hashMaps(){
	    let mut scores = HashMap::new();

		scores.insert(String::from("Blue"), 10);
		scores.insert(String::from("Yellow"), 40);

		//this is used to update the value of current key
		scores.insert(String::from("Blue") , 20);
		// using entry !
		scores.entry(String::from("Orange")).or_insert(50);
		// this will do nothing becuase key blue already exists
		scores.entry(String::from("Blue")).or_insert(10);

		let score = scores.get(&String::from("Blue")).copied().unwrap_or(0);
		println!("{:?} and \ncurrent score is {score}", scores);
}

