fn main() {
    println!("Himanshu Rust adventure :)!");
    let items = vec![
        "Come to Linkedin".to_string(),
        "Come to Twitter".to_string(),
    ];
    print_all(items);

    let circle = create_shape::<Circle>(true);
    circle.draw(); // This will draw a circle

    let square = create_shape::<Square>(false);
    square.draw(); // This will draw a square
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

trait Drawable {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

struct Square {
    side_length: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius {}", self.radius);
    }
}

impl Drawable for Square {
    fn draw(&self) {
        println!("Drawing a square with side length {}", self.side_length);
    }
}

// A generic function that can return any type implementing the Drawable trait
fn create_shape<T: Drawable>(is_circle: bool) -> T {
    if is_circle {
        Circle { radius: 5.0 }
    } else {
        Square { side_length: 4.0 }
    }
}
