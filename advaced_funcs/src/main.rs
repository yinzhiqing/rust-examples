
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn test_do_twice() {
    let answer = do_twice(add_one, 6);
    let answer1 = do_twice(|n|  n + 1, 5);

    println!("The answer is: {}", answer);
    println!("The answer1 is: {}", answer1);
}

fn test_map() {
    let list_of_numbers = vec![1, 2, 4];
    let list_of_strings: Vec<String> = 
        list_of_numbers.iter().map(|i| i.to_string()).collect();
    println!("{:?}", list_of_strings);
}

enum Status {
    Value(u32),
    Stop,
}

fn test_status() {
    let _list_of_statuses : Vec<Status> = (0u32..20).map(Status::Value).collect();
}

fn main() {

    test_do_twice();
    test_map();
    test_status();
}
