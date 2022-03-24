


fn largest<T: PartialOrd + Copy>(list: &[T])->T {
    let mut largest = list[0];
    for &item in list {
        if largest < item {
            largest = item;
        }
    }
    largest
}

fn test_largest() {
    let number_list = vec![1, 6, 3, 9, 14, 41, 34];
    let result = largest(&number_list);

    println!("max {} in {:?}", result, number_list);

    let number_list = vec!['a', 'c', 'b', 'x'];
    let result = largest(&number_list);
    println!("max {} in {:?}", result, number_list);
}

fn longest<'a>(x :&'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[test]
fn test_longest() {
    let x = "abc";
    let y : &'static str = "abcd";
    println!("longest: {}", longest(x, y));
}
fn main() {

    test_largest();

    test_longest();
}
