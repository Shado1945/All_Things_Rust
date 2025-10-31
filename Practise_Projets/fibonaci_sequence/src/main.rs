fn main() {
    // // Unoptimise
    // println!("This is to get the nth place of the fibonaci sequence");
    // let n = 6;
    // let mut second = 0;
    // let mut result = 1;
    // for fib in 0..n {
    //     println!("{} Fibonaci number = {}", fib + 1, result);
    //     if second == 0 {
    //         second = second + 1;
    //     } else {
    //         result = result + second;
    //         second = result - second;
    //     }
    // }

    // // optimised
    println!("This is to get the nth place of the fibonaci sequence");
    let n = 6;
    let (mut second, mut result) = (0, 1);
    for fib in 1..=n {
        println!("{} Fibonaci number = {}", fib, result);
        let temp = second + result;
        second = result;
        result = temp;
    }
}
