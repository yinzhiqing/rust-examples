fn main() {
    let mut r = String::from("hello");
    let r1 = &r;
    let r2 = &r;
    println!("{}, {}", r1, r2);
    let r3 = &mut r;
    println!("{}", r3);
}
