pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    _width: u32,
    _height: u32,
}

impl Rectangle {
    fn _can_hold(&self, other: &Rectangle) -> bool {
        self._width > other._width && self._height > other._height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    _value: i32,
}

pub fn prints_and_return(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

impl Guess {
    pub fn new(_value: i32) -> Guess {
        if _value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.", _value);
        } else if _value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.", _value);
        }

        Guess { _value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_return(4);
        assert_eq!(10, value);
    }

    #[test]
    #[ignore]
    fn this_test_will_fail() {
        let value = prints_and_return(8);
        assert_eq!(5, value);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2+2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn it_works0() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            _width: 8,
            _height: 7,
        };
        let smaller = Rectangle {
            _width: 5,
            _height: 1,
        };

        assert!(larger._can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            _width: 8,
            _height: 7,
        };
        let smaller = Rectangle {
            _width: 5,
            _height: 1,
        };

        assert!(!smaller._can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_ne!(3, add_two(2));
    }
    
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carlos");
        assert!(result.contains("Carlos"), 
        "Greeting did not contain name, value was `{}`", result);
    }

    #[test]
    #[should_panic(expected = "greater than or equal to 1")]
    fn greater_than_1() {
        Guess::new(-200);
    }
}
