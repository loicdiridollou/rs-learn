// error handling in Rust

use std::{fs::File, io::ErrorKind};

fn main() {
    // panic!("This is an error crashing the program")

    enum _Result<T, E> {
        Ok(T),
        Err(E),
    }

    let file = File::open("hello.txt");
    let _f = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(error) => panic!("{:?}", error),
            },
            error_kind => {
                panic!("Other error {:?}", error_kind)
            }
        },
    };
}
