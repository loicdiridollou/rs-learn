use std::collections::HashMap;

fn main() {
    vectors();
    strings();
    hashmaps();
}

fn hashmaps() {
    let blue = String::from("blue");
    let red = String::from("red");
    let mut new_map: HashMap<String, i32> = HashMap::new();

    new_map.insert(blue, 10);
    new_map.insert(red, 15);

    let team_name = String::from("blue");
    println!("{:?}", new_map.get(&team_name).unwrap());

    for (key, val) in &new_map {
        println!("{}, {}", key, val);
    }

    let text: String = String::from("this is a new string or is this");
    let mut counter: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        let count = counter.entry(word).or_insert(0);
        *count += 1;
    }
    println!("Number of word this: {}", counter.get("this").unwrap());
    println!("Number of word new: {}", counter.get("new").unwrap());
    println!("{:?}", counter);
}

fn strings() {
    // strings are a collection of utf-8 bytes
    let _s1 = String::from("Hello world");
    let s2 = "grve3gr";
    let _s3 = s2.to_string();
    let mut s4 = String::from("grve3gr");

    s4.push_str("additional");
    s4.push('!');
    println!("{}", s4);

    let s5 = String::from("Hello");
    let s6 = String::from("world");
    let s7 = format!("{} {}", s5, s6);
    println!("{}", s7);
}

fn vectors() {
    let a: [i32; 3] = [1, 2, 3];
    println!("{:?}", a);

    let mut v: Vec<i32> = Vec::new();
    v.push(2);
    println!("{:?}", v);

    {
        let v2: Vec<i32> = vec![4, 5, 6];
        println!("{:?}", v2)
    }
    // println!("{:?}", v2) won't work as out of scope

    let third = &a[2];
    println!("{}", third);

    // match
    match v.get(0) {
        Some(third) => println!("Third {}", third),
        None => println!("Number not found"),
    }

    let mut new_v: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    let third = &new_v[3];
    println!("The fourth item of the vector is: {}", third);
    new_v.push(5);

    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];

    for i in &mut v {
        println!("{}", i);
        *i += 50;
    }
    println!("{:?}", v);

    let v: Vec<i32> = vec![1, 2, 3, 4, 5];

    for i in &v[..3] {
        println!("{}", i);
    }
    println!("{}", v.len());
    println!("{:?}", v);

    enum SpreadSheetCell {
        Int(i32),
        Text(String),
        Float(f64),
    }

    let row: Vec<SpreadSheetCell> = vec![
        SpreadSheetCell::Text(String::from("red")),
        SpreadSheetCell::Float(4.50),
        SpreadSheetCell::Int(42),
    ];

    match &row[0] {
        SpreadSheetCell::Text(string) => println!("The text is {}", string),
        _ => println!("Not a text"),
    };
}
