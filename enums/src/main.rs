/*
enum IpAddrKind {
    V4,
    V6
}
*/
enum IpAddr {
    V4(String),
    V6(String),
}

impl IpAddr {
    fn call(&self) {
        //self.push_str("...");
        println!("self: {}", "self");
    }
}
fn main() {

    let m = IpAddr::V4(String::from("127.0.0.1"));
    let _m6 = IpAddr::V6(String::from("::1"));
    m.call();
}
