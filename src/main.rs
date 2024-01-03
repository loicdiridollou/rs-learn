use std::{cmp::Ordering, io::stdin};

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Plesse enter your guess:");

    loop {
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Failed to read guess");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Equal => {
                println!("Perfect {secret_number} that is the number!");
                break;
            }

            Ordering::Greater => println!("Too high!"),
        }
    }
}
