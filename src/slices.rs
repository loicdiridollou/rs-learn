pub fn slices_main() {
    let _word: String = String::from("Test value");

    println!("{}", first_word(&_word));
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
