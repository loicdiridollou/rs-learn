pub fn control_flow() {
    // if else
    let num: i32 = 48;
    if num < 10 {
        println!("Number is less than 10.");
    } else if num < 30 {
        println!("Number is less than 30");
    } else {
        println!("Number is more than 30");
    }

    let condition = false;
    let number = if condition { 5 } else { 6 };
    println!("Number value is {}", number);

    // loop
    let mut cntr = 0;
    loop {
        println!("In the loop");
        if cntr >= 5 {
            break;
        }
        cntr += 1;
    }

    // while loop
    while cntr > 0 {
        println!("In the loop with counter value {}", cntr);
        cntr -= 1;
    }

    // for loop
    let a: [i32; 6] = [0, 1, 2, 3, 4, 5];
    for element in a {
        println!("{}", element);
    }

    for element in a.iter() {
        println!("{}", element);
    }

    for element in -5..-2 {
        println!("{}", element);
    }

    let b = &a[2..4];
    println!("{:?}", b);
}
