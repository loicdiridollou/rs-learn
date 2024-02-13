fn main() {
    vectors()
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
