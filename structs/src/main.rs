struct User {
    active : bool,
    username: String,
    email: String,
    sign_in_count: u64

}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn update_width(&mut self, width: u32) {
        self.width = width;
    }
}

fn print_split_line() {
    println!("--------------------------");
}


fn test_user() {
    let user1 = User {
        active : true,
        username: String::from("user1"),
        email: String::from("user1@example.com"),
        sign_in_count: 1
    };

    println!("email: {}", &user1.email);
    println!("username: {}", &user1.username);

    print_split_line();

    let user2 = User {
        //no set, user1.username can't use
        username: String::from("user2"),
        email: String::from("user2@example.com"),
        ..user1
    };
    println!("email: {}", &user2.email);
    println!("username: {}", &user2.username);
    // show
    println!("username: {}", &user1.username);
}

fn test_color() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("r: {}", black.0);
    println!("x: {}", origin.0);
}

fn test_rect() {
    let mut rect1 = Rectangle { 
        width : 10,
        height: 30
    };
    println!("rect1 area: {:#?}", rect1.area());

    rect1.update_width(20);
    println!("rect1 area: {:#?}", rect1.area());


}
fn main() {
    test_user();
    print_split_line();
    test_color();
    print_split_line();
    test_rect();
}
