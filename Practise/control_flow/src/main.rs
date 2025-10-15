fn main() {
    // if expression
    let number = 9;
    if number > 3 {
        println!("The number is bigger than 3");
    }
    // if else expression
    let number2 = 3;
    if number2 < 5 {
        println!("Condition was true");
    } else {
        println!("condition was false");
    }

    // else if
    //Only executes the block of the first true condition
    let number3 = 6;
    if number3 % 4 == 0 {
        println!("number is divisable by 4");
    } else if number3 % 3 == 0 {
        println!("number is divisable by 3");
    } else if number3 % 2 == 0 {
        println!("number is divisable by 2");
    } else {
        println!("number is not divisable by 4, 3 or 2");
    }

    //if in a let statement
    let condition = true;
    let number4 = if condition { 5 } else { 6 };
    println!("The value of number4 is: {}", number4);
}
