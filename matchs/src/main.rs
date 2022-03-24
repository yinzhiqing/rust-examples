#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

enum IpAddr {
    V4(String),
    V6(u32),
}

fn value_in_ia(addr: IpAddr) -> u32 {
    match addr {
        IpAddr::V4(v) => {
            println!("type is v4");
            4
        }
        IpAddr::V6(v) => {
            println!("type is v6");
            6
        }
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(usstate) => {
            println!("State Quarter from {:?}", usstate);
            25
        }
    }
}

fn main() {
    println!("coin: {}", value_in_cents(Coin::Penny));
    println!("coin: {}", value_in_cents(Coin::Nickel));
    println!("coin: {}", value_in_cents(Coin::Dime));
    println!("coin: {}", value_in_cents(Coin::Quarter(UsState::Alabama)));

    println!("coin: {}", value_in_ia(IpAddr::V4(String::from("127.0.0.1"))));
    println!("coin: {}", value_in_ia(IpAddr::V6(32)));

    let coin = Coin::Dime;
    let coin = Coin::Quarter(UsState::Alaska);
    if let Coin::Quarter(usstate) = coin {
       println!("State Quarter from {:?}", usstate);
    } else {
        println!("other type ");
    }
}

