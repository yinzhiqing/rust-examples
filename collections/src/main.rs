#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn vec_test() {
    let mut v:Vec<i32> = Vec::new();
    v.push(10);
    let first = &v[0];
    println!("vec {:?}", first);

    for i in &mut v {
        *i += 3;
        println!("{}", i);
    }
    println!("vec {:?}", v);


    let v = vec![1,3,5];
    println!("vec {:?}", v);

    match v.get(3) {
        Some(value) => {
            println!("elet 2 is {}", value);
        }
        None => {
            println!("None");
        }
        }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(1.0345),
        SpreadsheetCell::Text(String::from("blue")),
    ];
    println!("row {:?}", row);
    for i in &row {
        match i {
            SpreadsheetCell::Int(v) => println!("int value: {}", v),
            SpreadsheetCell::Float(v) => println!("float value: {}", v),
            _ => println!("other"),
        }
    }
}


fn test_string() {
    let mut s = String::from("lo");
    s.push('l');
    println!("str {:?}", s);
    s.push_str(" hello.");
    println!("str {:?}", s);

    let hello = String::from("hello, ");
    let world = String::from("world!");
    let s3 = hello + &world;
    println!("S3 : {}", s3);

    for c in "नमस्ते".chars() {
        println!("{}", c)
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b)
    }
}
 
fn test_map() {
    use std::collections::HashMap;
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("scores: {:?}", scores);

    let keys = vec![String::from("blue"), String::from("yellow")];
    let values = vec![10, 30];
    let scoresn: HashMap<_,_> = keys.into_iter().zip(values.into_iter()).collect();
    println!("vec -> map scores: {:?}", scoresn);

    let name = String::from("lihong");
    let addr = String::from("beijing");
    let mut map = HashMap::new();
    map.insert(name, addr);
    println!("name-addr: {:?}", map);

}

fn test_read_username_from_file() {
    let d = read_username_from_file();
    if let Ok(name) = d {
        println!("{:?}", name);
    } else {
        println!("{:?}", d);
    }
}

fn last_line(text: &str)->Option<&str>{
    //text.lines().next()?.chars().last()
    let l = text.lines().last()?;
    Some(l)
    
}
fn test_read_username_from_file2() {
    let d = read_username_from_file2();
    if let Ok(name) = d {
        println!("{:?}", name);
        for n in name.lines() {
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn vec_test() {
    let mut v:Vec<i32> = Vec::new();
    v.push(10);
    let first = &v[0];
    println!("vec {:?}", first);

    for i in &mut v {
        *i += 3;
        println!("{}", i);
    }
    println!("vec {:?}", v);


    let v = vec![1,3,5];
    println!("vec {:?}", v);

    match v.get(3) {
        Some(value) => {
            println!("elet 2 is {}", value);
        }
        None => {
            println!("None");
        }
        }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(1.0345),
        SpreadsheetCell::Text(String::from("blue")),
    ];
    println!("row {:?}", row);
    for i in &row {
        match i {
            SpreadsheetCell::Int(v) => println!("int value: {}", v),
            SpreadsheetCell::Float(v) => println!("float value: {}", v),
            _ => println!("other"),
        }
    }
}


fn test_string() {
    let mut s = String::from("lo");
    s.push('l');
    println!("str {:?}", s);
    s.push_str(" hello.");
    println!("str {:?}", s);

    let hello = String::from("hello, ");
    let world = String::from("world!");
    let s3 = hello + &world;
    println!("S3 : {}", s3);

    for c in "नमस्ते".chars() {
        println!("{}", c)
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b)
    }
}
 
fn test_map() {
    use std::collections::HashMap;
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("scores: {:?}", scores);

    let keys = vec![String::from("blue"), String::from("yellow")];
    let values = vec![10, 30];
    let scoresn: HashMap<_,_> = keys.into_iter().zip(values.into_iter()).collect();
    println!("vec -> map scores: {:?}", scoresn);

    let name = String::from("lihong");
    let addr = String::from("beijing");
    let mut map = HashMap::new();
    map.insert(name, addr);
    println!("name-addr: {:?}", map);

}

fn test_read_username_from_file() {
    let d = read_username_from_file();
    if let Ok(name) = d {
        println!("{:?}", name);
    } else {
        println!("{:?}", d);
    }
}

fn last_line(text: &str)->Option<&str>{
    //text.lines().next()?.chars().last()
    let l = text.lines().last()?;
    Some(l)
    
}
fn test_read_username_from_file2() {
    let d = read_username_from_file2();
    if let Ok(name) = d {
        println!("{:?}", name);
        for n in name.lines() {
            println!("{:?}", n);
        }
        println!("last {:?}", last_line(&name));
    } else {
        println!("{:?}", d);
    }
}
use std::io::{self, Read};
fn read_username_from_file() -> Result<String, io::Error> {
    use std::fs::File;
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    println!("read data from hello.txt");
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    use std::fs::File;
    let mut s = String::new();
    /* can use 
    File::open("hello.txt")?.read_to_string(&mut s)?;
    return Ok(s);
    */

    let mut f = File::open("hello.txt")?;
    println!("read data from hello.txt -> 2");
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn test_unwrap() {
    use std::fs::File;
    //let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("test expect");
}

fn split_line(func:&str) {
    println!("-------------------{}------------------------", func);
}
fn main() {
    println!("Hello, conllections!");
    vec_test();
    split_line("test_string");
    test_string();
    split_line("test_map");
    test_map();
    split_line("test_read_username_from_file");
    test_read_username_from_file();
    split_line("test_read_username_from_file2");
    test_read_username_from_file2();
    split_line("test_unwrap");
    test_unwrap();
    split_line("end");
}
            println!("{:?}", n);
        }
        println!("last {:?}", last_line(&name));
    } else {
        println!("{:?}", d);
    }
}
use std::io::{self, Read};
fn read_username_from_file() -> Result<String, io::Error> {
    use std::fs::File;
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    println!("read data from hello.txt");
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    use std::fs::File;
    let mut s = String::new();
    /* can use 
    File::open("hello.txt")?.read_to_string(&mut s)?;
    return Ok(s);
    */

    let mut f = File::open("hello.txt")?;
    println!("read data from hello.txt -> 2");
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn test_unwrap() {
    use std::fs::File;
    //let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("test expect");
}

fn split_line(func:&str) {
    println!("-------------------{}------------------------", func);
}
fn main() {
    println!("Hello, conllections!");
    vec_test();
    split_line("test_string");
    test_string();
    split_line("test_map");
    test_map();
    split_line("test_read_username_from_file");
    test_read_username_from_file();
    split_line("test_read_username_from_file2");
    test_read_username_from_file2();
    split_line("test_unwrap");
    test_unwrap();
    split_line("end");
}
