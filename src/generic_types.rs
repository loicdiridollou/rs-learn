// generic types in Rust

fn main() {
    let list_nums = vec![1, 2, 3, 695, 35];
    println!("{}", largest_num(&list_nums));

    let list_char = vec!['y', 'm', 'a', 'q'];
    println!("{}", largest_char(&list_char));

    let list_nums = vec![1, 2, 3, 695, 35];
    println!("{}", largest(&list_nums));

    let list_char = vec!['y', 'm', 'a', 'q'];
    println!("{}", largest(&list_char));
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
