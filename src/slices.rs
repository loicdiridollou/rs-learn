pub fn slices_main() {
    let mut word: String = String::from("Test value");
    let hello: &str = &word[0..4];
    let world: &str = &word[5..10];
    println!("{}", hello);
    println!("{}", world);

    println!("{}", first_word(&word));
    println!("{}", first_word_str(&word)); // also works by coarced String to str
    word.clear();
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    return s.len();
}

fn first_word_str(s: &str) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    return s.len();
}
