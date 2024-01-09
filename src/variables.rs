pub fn variables_main() {
    mutability();
    let val = constant();
    println!("Constant value is {}", val);
    shadowing();
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

fn constant() -> u32 {
    const TEST: u32 = 100_000;
    return TEST;
}

fn shadowing() {
    let x = 6;
    println!("The value of x is {}", x);
    // x = 5 would not work
    let x = 6;
    println!("The value of y is {}", x);
}
