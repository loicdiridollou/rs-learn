pub fn ownership_main() {
    fn a() {
        let x: &str = "Hello";
        let y: i32 = 34;
        println!("{}, {}", x, y);
        b();
    }

    fn b() {
        let _x = String::from("world");
    }

    a();

    let s: String = String::from("Hello");
    takes_ownership(s);

    // println!("{}", s) can't be borrowed as flushed in the takes_ownership
}

fn takes_ownership(s: String) {
    println!("{}", s);
}
