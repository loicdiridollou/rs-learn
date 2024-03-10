pub fn slices_main() {
    slices_on_string();
    slices_on_array();
    squares_of_sorted();
}

fn squares_of_sorted() {
    let values = Vec::<i32>::from([-9, -4, 0, 1, 10]);
    let (mut l, mut r): (usize, usize) = (0, values.len() - 1);
    let mut new_vec: Vec<i32> = vec![0; values.len()];
    let mut i = values.len() - 1;

    while l <= r {
        if values[l].pow(2) > values[r].pow(2) {
            new_vec[i] = values[l].pow(2);
            l += 1;
            if i == 0 {
                break;
            }
            i -= 1
        } else {
            new_vec[i] = values[r].pow(2);
            r -= 1;
            if i == 0 {
                break;
            }
            i -= 1;
        }
    }

    println!("{:?}", new_vec);
}

fn slices_on_array() {
    let a: [i8; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", &a[0..2]);
}

fn slices_on_string() {
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

    s.len()
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
