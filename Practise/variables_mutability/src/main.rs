fn main() {
    //Immutable variables
    let x = 5;
    println!("The value of x is: {}", x);
    // Thie code below will break as this variable
    // is immutable
    // x = 6;
    // println!("The value of x is: {}", x);

    //Mutable
    let mut y = 6;
    println!("The value of y is: {}", y);
    y = 8;
    println!("The value of y is now: {}", y);

    //Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!(
        "The value of three hours in seconds is: {}",
        THREE_HOURS_IN_SECONDS
    );

    //Shadowing
    let s = 5;
    let s = s + 1;
    {
        let s = s * 2;
        println!("The value of s in an inner scope is: {}", s);
    }
    println!("The value of s in an outer scope is: {}", s);
    //Other shadowing allows the variable to change the type
    //Note does not work for mutable variables
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The length of spaces is: {}", spaces);
}
