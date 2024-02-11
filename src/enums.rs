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
}
