pub fn var_main() {
    mutability()
}

fn mutability() {
    let x = 6;
    println!("The value of x is {}", x);
    // x = 5 would not work
    let mut y = 6;
    println!("The value of y is {}", y);
    y = 5;
    println!("The value of y is {}", y);
}
