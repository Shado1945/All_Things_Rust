fn main() {
    // loop
    // Infinite until explicitly stopped
    let mut counter = 0;
    let result = loop {
        println!("Count: {}", counter);
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("The result is {result}");

    //loop with a return inside
    println!("\n Second loop");
    let mut count1 = 0;
    'counting_up: loop {
        println!("count = {}", count1);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count1 == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count1 += 1;
    }
    println!("End count = {count1}");

    // while loop
    println!("\nWhile loop");
    let mut num1 = 3;
    while num1 != 0 {
        println!("{num1}");
        num1 -= 1;
    }
    println!("Lift off !!!!");

    //for loop
    println!("\nFor loop");
    let array1 = [10, 20, 30, 40, 50];

    for element in array1 {
        println!("The value is: {}", element);
    }

    // for loop with reverse range
    for rng in (1..4).rev() {
        println!("{rng}");
    }
    println!("LIFTOFF !!!");
}
