// generic types in Rust

use std::fmt;

fn main() {
    let list_nums = vec![1, 2, 3, 695, 35];
    println!("{}", largest_num(&list_nums));

    let list_char = vec!['y', 'm', 'a', 'q'];
    println!("{}", largest_char(&list_char));

    let list_nums = vec![1, 2, 3, 695, 35];
    println!("{}", largest(&list_nums));

    let list_char = vec!['y', 'm', 'a', 'q'];
    println!("{}", largest(&list_char));

    let coordinate = Coord { x: 1, y: 2 };
    println!("{}", coordinate);
}

struct Coord<T> {
    x: T,
    y: T,
}

impl<T: fmt::Display> fmt::Display for Coord<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Coordinate with x: {} and x: {}", &self.x, &self.y)
    }
}

fn largest_num(s: &[i32]) -> &i32 {
    let mut largest: &i32 = &s[0];

    for item in s {
        if item > largest {
            largest = item
        }
    }
    return largest;
}

fn largest_char(s: &[char]) -> &char {
    let mut largest: &char = &s[0];

    for item in s {
        if item > largest {
            largest = item
        }
    }
    return largest;
}

fn largest<T: PartialOrd>(s: &[T]) -> &T {
    let mut largest: &T = &s[0];

    for item in s {
        if item > largest {
            largest = item
        }
    }
    return largest;
}
