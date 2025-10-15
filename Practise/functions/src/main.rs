//Main is also a function
fn main() {
    println!("Hello, world!");
    //First additional function
    second_method();
    //Function with params
    function_with_params(5);
    //Function with diferent types
    function_with_diff_typ(5, 'h');

    //Statements and expressions
    let y = {
        //this is the expression
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    //Functions with Return values
    let five = five();
    println!("The value of five is: {}", five);

    let plus_one = plus_one(5);
    println!("The value of plus_one: {}", plus_one);
}

//First additional function
fn second_method() {
    println!("Another function.");
}

//Function with params
fn function_with_params(x: i32) {
    println!("The value of x is: {}", x);
}

//Function with diferent types
fn function_with_diff_typ(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

//Functions with Return values
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
