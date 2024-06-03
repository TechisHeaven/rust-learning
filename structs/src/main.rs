
fn main() {

     //create_structs();
     //refacetoring_with_struct();
     //method_with_more_params();
    //method_structure();

   


    let circle = Shape::Circle(40.0);
    let square = Shape::Square(50.0);
    let rectangle = Shape::Rectangle(40.0, 30.0);


    let print =  calculate_shape(rectangle);
    println!("{}", print);


}


fn calculate_shape(shape: Shape)-> f64 {
    match shape {
        //Shape::Circle(radius) => 3.14 * radius * radius,
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Square(radius) => radius * radius,
        Shape::Rectangle(lenght, width) => lenght * width,
    }
}

 enum Shape {
    Circle(f64),
   Square(f64),
   Rectangle(f64 , f64)
}




    struct User{
    active: bool,
    username: String,
    email : String,
    sign_in_count: u64,
}



fn create_structs(){
    let user1 = User {
        active: false,
        username: String::from("someusername"),
        email: String::from("someEmail@gmail.com"),
        sign_in_count: 1,
    };


    let user2 = User {
        active: user1.active,
        username: String::from("someotherusername"),
        ..user1
    };
    
    let user3 = User {
        active : user1.active,
        username: String::from("someothermoreusername"),
        ..user2
    };

    println!("username = {} {} {} {}"  , user3.username, user3.active , user3.email , user3.sign_in_count);
}

fn refacetoring_with_struct (){ 
    // if want to print whole decleared variable structure #[derive(Debug)]
    
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect1 = Rectangle {
        width : 30,
        height: 50,
    };

    // dbg!(&rect1);
    // for whole printing array println!("Area of Rectangle is {:#?}" , rect1);
    println!("Area of Reactangle is {}", area(&rect1));

    fn area(dimensions: &Rectangle) -> u32 {
        dimensions.width * dimensions.height
    }

}


fn method_structure(){
    #[derive(Debug)]
    struct Rectangle { 
        width : u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
        fn width(&self) -> bool {
            self.width > 0
        }
    }

    let rect1 = Rectangle {
        width: 0, 
       height: 20,
    };

    if rect1.width() {
        println!("The area of second Rectangle is {} square pixels", rect1.area());
    }
    else{
    println!("Width of rectangle is equal to zero {}", rect1.width);
    }

}


fn method_with_more_params(){
    struct Rectangle { 
        width : u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
        fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
        }
    }
    impl Rectangle {
         fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }

    let rect1 = Rectangle {
        width: 50, 
        height: 20,
    };
    let rect2 = Rectangle {
        width: 20, 
        height: 10,
    };

    println!("rect1 can hold rect2  {}" , rect1.can_hold(&rect2));
}