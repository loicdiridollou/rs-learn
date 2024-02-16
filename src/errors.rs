// error handling in Rust

use std::io::ErrorKind;
use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    // panic!("This is an error crashicag the program")

    error_propagation();
    error_handling_file();
}

fn error_propagation() {
    match read_from_file() {
        Ok(_) => println!("Read the file"),
        Err(e) => println!("{:?}", e),
    }
}

fn read_from_file() -> Result<String, io::Error> {
    let f_result = File::open("hello.txt");

    let mut f = match f_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s: String = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn error_handling_file() {
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
    let _file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| panic!("{:?}", error))
        } else {
            panic!("{:?}", error);
        }
    });
}
