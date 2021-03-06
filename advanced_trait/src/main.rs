
use std::ops::Add;


struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output =   Millimeters;
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

fn test_add() {
    let mm = Millimeters(1);
    let m = Meters(2);
    let added = mm + m;
    println!("millimeters + meters : {}", added.0);
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn test_same_funcs() {
    let person = Human;
    person.fly();

    Pilot::fly(&person);
    Wizard::fly(&person);

}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn test_dog() {
    println!("A baby dog is called a {}", Dog::baby_name());
    println!("完全限定语法：
       <Type as Trait>::function(receiver_if method, next_arg, ...);
        ");
    println!("[完全限定语法]A baby dog is called a {}", <Dog as Animal>::baby_name());
}

use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();

        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl OutlinePrint for Point {}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(","))
    }
}

fn test_wrapper() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

//类型别名
type Thunk = Box<dyn Fn() + Send + 'static>;


fn takes_long_type(f: Thunk) {
    let _f: Thunk = Box::new(|| println!("hi"));
}

fn main() {
    assert_eq!(Point{x: 1, y: 0} + Point{x: 2, y: 3},
        Point{x:3, y:3}
        );

    test_same_funcs();
    test_dog();
    let pot = Point{x: 1, y:3};
    pot.outline_print();

    test_wrapper();

    test_add();
}
