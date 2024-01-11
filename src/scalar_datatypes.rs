pub fn scalar_types() {
    // integer
    let _a: i32 = 10_000;
    let _b: i32 = 0xff;
    let _c: i32 = 0o77;
    let _d: i32 = 0b1100_0000;
    let _e: u8 = b'A';
    // let _f: u8 = 256; would not work as does not fit on 8 bits

    // floating point numbers
    let _g = 5489.48; // inferred type is f64 as architecture of the m1
    let _h: f32 = 49302.48;

    // booleans
    let _t: bool = true;
    let _f: bool = false;

    // characters
    let _y: char = 'A';
    // let z: char = 'dA' would not work as should be one character long
}

pub fn compound_types() {
    let _tup: (&str, f32) = ("John", 43.5);
    let (_v1, _v2) = _tup;
    let _v3 = _tup.1;

    let byte = [0; 8];
    println!("{:?}", byte)
}
