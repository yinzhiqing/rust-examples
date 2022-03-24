#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exploration() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic]
    fn another() {
        panic!("Make this test panic!");
    }

    #[test]
    #[should_panic]
    fn another_than() {
        panic!("Make this test panic!");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width : 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height:1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    #[ignore]
    fn smaller_cannt_hold_larger_ign() {
        let larger = Rectangle {
            width : 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height:1,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        println!("this is println outout <<<<<<<<<<");
        let g = Guess::new(101);
        assert!(g.value > 0);
    }
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

pub struct Guess {
    pub value : i32
}

impl Guess {
    pub fn new(value: i32)-> Guess {
        if value < 1 || value > 100 {
            println!("this is println outout <<<<<<<<<<");
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}
