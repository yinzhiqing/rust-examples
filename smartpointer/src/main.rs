
fn test_list() {
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    
    use List::{Cons, Nil};

    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

fn test_deref() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn test_deref_box() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}


//*******************mybox****************
use std::ops::Deref;
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn test_mybox() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
//*******************drop*****************
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'", self.data);
    }
}

fn test_drop() {
    let c = CustomSmartPointer{
        data: String::from("my stuff"),
    };

    let _d = CustomSmartPointer{
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers Created.");
    drop(c);
}

//***************************************
fn test_rc() {
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }
    use List::{Cons, Nil};
    use std::rc::Rc;

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}. ", Rc::strong_count(&a));
    let _c = Cons(4, Rc::clone(&a));
    println!("count after creating c = {}", Rc::strong_count(&a));
    {
        let _d = Cons(5, Rc::clone(&a));
        println!("count after creating d = {}", Rc::strong_count(&a));
    }
    println!("count after d goes out of scope = {}", Rc::strong_count(&a));
}
fn main() {
    test_list();
    test_deref();
    test_deref_box();
    test_mybox();
    test_drop();
    test_rc();
}
