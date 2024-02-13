// Module for enums and pattern matching

#[derive(Debug)]
enum IpAddrType {
    V4(String),
    V6(String),
}

// struct IpAddr {
//     kind: IpAddrType,
//     address: String,
// }

enum Message {
    Quit,
    ChangeColor { r: i32, g: i32, b: i32 },
}

impl Message {
    fn some_function() {
        println!("Test Message")
    }

    fn change_color(&self) {
        match self {
            Message::ChangeColor { r, g, b } => println!("{} {} {}", r, g, b),
            Message::Quit => println!("Wrong type of message"),
        }
    }
}

fn main() {
    let localhost: IpAddrType = IpAddrType::V4(String::from("127.0.0.1"));
    let localhost_v6: IpAddrType = IpAddrType::V6(String::from("n3f4:tv54:32vf:st53"));

    println!("{:?}", localhost);
    println!("{:?}", localhost_v6);

    let _message: Message = Message::ChangeColor {
        r: 32,
        g: 35,
        b: 69,
    };

    _message.change_color();
    Message::some_function();
    let _quit_message = Message::Quit;

    // option enums
    option_enum();

    // match patterns
    match_patterns();

    // match on option enum
    option_match_enum();
}

fn option_enum() {
    let _some_num = Some(34);
    let _some_string = Some("string");
    let _some_empty: Option<i32> = None;

    // summing optional and number
    let num: i8 = 5;
    let option_num: Option<i8> = Some(6);

    println!("{}", num + option_num.unwrap_or(0));
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn match_patterns() {
    let value = get_coin_value(&Coin::Dime);
    println!("{}", value);

    let value = get_coin_value(&Coin::Penny);
    println!("{}", value);

    let value = get_coin_value(&Coin::Nickel);
    println!("{}", value);

    let value = get_coin_value(&Coin::Quarter);
    println!("{}", value);
}

fn get_coin_value(coin: &Coin) -> i32 {
    let value = match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    };
    return value;
}

fn option_match_enum() {
    let five = Some(5);
    let six = plus_one(five);
    println!("{:?}", six);
    println!("{:?}", five);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
