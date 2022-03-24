#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


pub trait HelloMacro {
    fn hello_macro();
}

/*
use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {

    fn hello_macro() {
        println!("Hello, Macro! My name is pncakes!");
    }
}
*/
