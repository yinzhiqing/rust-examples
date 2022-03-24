
static HELLO_WORLD: &str = "hello world!";
static mut COUNTER: i32 = 0;

fn test_unsafe() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    println!("r1 address: {:?}", r1);
    println!("r2 address: {:?}", r2);
    unsafe {
        println!("r1: {:?}", *r1);
        println!("r2: {:?}", *r2);
    }
}

unsafe fn dangerous() {
    println!("call unsafe dangerous");
}

unsafe fn test_dangerous() {
    dangerous();
}

use std::slice;
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!( mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}   

fn test_extern() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

fn test_static() {
    println!("name is {}", HELLO_WORLD);


    unsafe {
        println!("counter before: {}", COUNTER);

        COUNTER += 2;

        println!("counter after: {}", COUNTER);
    }
}

fn main() {
    test_unsafe();
    unsafe {
        test_dangerous();
    }

    test_extern();
    test_static();

}

