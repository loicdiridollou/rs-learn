use std::fmt::Display;

fn main() {
    understand_lifetimes();
    dangling_ref();
    dangling_ref_inner_scope();
    lifetimes_with_struct();
    let res = longest_with_an_announcement("terst", "589", "Hello world!");
    println!("{}", res);
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

fn dangling_ref_inner_scope() {
    let s1: String = String::from("abgjroe");
    let result: &str;
    {
        let s2: String = String::from("gjnre");
        result = longest(&s1, &s2);
        println!("{}", result)
    }
}

fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() > right.len() {
        left
    } else {
        right
    }
}

fn lifetimes_with_struct() {
    struct Importance<'a> {
        part: &'a str,
    }

    let first_sent = String::from("Test");
    let _i = Importance {
        part: &first_sent.to_string(),
    };

    println!("{}", _i.part);
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
