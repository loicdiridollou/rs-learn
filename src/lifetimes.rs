fn main() {
    understand_lifetimes();
    dangling_ref();
}

fn understand_lifetimes() {
    let r: &i32;
    let x: i32 = 5;
    r = &x;

    println!("{}", r)
}

fn dangling_ref() {
    let s1: String = String::from("abgjroe");
    let s2: String = String::from("gjnre");

    println!("{}", longest(&s1, &s2))
}

fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() > right.len() {
        left
    } else {
        right
    }
}
