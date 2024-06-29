pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 5,
            height: 4,
        };

        let smaller = Rectangle {
            width: 2,
            height: 1,
        };
        let larger_can_hold = larger.can_hold(&smaller);
        if !larger_can_hold {
            panic!("larger can't hold");
            // assert!(larger.can_hold(&smaller));
        }
        assert!(larger_can_hold);
    }
}

pub fn add_two(a: i32) -> i32 {
    return a + 2;
}

#[cfg(test)]
mod tests3 {
    use super::*;

    #[test]
    fn it_add_two() {
        assert_eq!(4, add_two(2));
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

#[cfg(test)]
mod tests4 {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol2"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
}
