//https://kaisery.github.io/trpl-zh-cn/ch18-03-pattern-syntax.html
fn test_ifelse() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color.");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

fn test_whilelet() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);

    while let Some(v) = stack.pop() {
        println!("{}", v);
    }
}

fn test_for() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

fn test_match() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) if y < 20 => println!("Matched, y = {:?}", y),

        _=> println!("Default case, x = {:?}", x),

    }

    println!("at the end: x = {:?} y = {:?}", x, y);
}

fn test_match_range() {
    let x = 11;

    match x {
        1..=10 => println!("at 1 ..10"),
        _ => println!("out of range(1..10)"),
    }
}

fn test_match_if() {
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        10..=20 => println("10~20"),
        _ => println!("no"),
    }
}

fn print_line() {
    println!("-----------------------------");
}
fn main() {
    test_ifelse();
    print_line();
    test_whilelet();
    print_line();
    test_for();
    print_line();
    test_match();
    print_line();
    test_match_range();
    print_line();
    test_match_if();

}
