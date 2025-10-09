fn main() {
    //Scaler types
    //Rust has four scaler types
    //Integer, Floating-point numbers, booleans and characters

    //INTEGERS
    // - signed or unsigned
    // signed: number can be negative
    // unsigned: number can only be postive
    //Rust defaults to i32
    let i: i32 = -5;
    let u: u32 = 6;
    println!("signed integer: {}, unsigned integer: {}", i, u);

    // FLOATS
    // two types f32 and f64
    let f_32: f32 = 3.2;
    let f_64 = 2.3;
    println!("f32 type: {}, f64 type: {}", f_32, f_64);

    //BOOLEANS
    let t = true;
    let f: bool = false;
    println!("t val: {}, f val: {}", t, f);

    //CHARACTER
    let c = 'z';
    let z: char = 'z';
    println!("char c: {}, char z: {}", c, z);
}
