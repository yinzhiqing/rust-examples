fn main() {
    println!("Hello, world!");

    for number in (1..5).rev() {
        println!("{}", number);
    }

    println!("--------------------------");

    let a = [1, 2, 3, 5, 8];
    for element in a {
        println!("{}", element);
    }

}
