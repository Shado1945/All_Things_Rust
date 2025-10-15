fn main() {
    //Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
    println!("The value of x is: {x}");
    println!("The value of z is: {z}");

    let tup2: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = tup2.0;
    let six_point_four = tup2.1;
    let one = tup2.2;

    println!("Values: {}, {}, {}", five_hundred, six_point_four, one);

    //Array
    let one_a = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let two_a: [i32; 5] = [1, 2, 3, 4, 5];
    let three_a = [5; 3];

    println!("First element in the array one_a: {}", one_a[0]);
    println!(
        "First, 5th and last month: {}, {}, {}",
        months[0], months[4], months[11]
    );
    println!("Third value of array two_a: {}", two_a[2]);
    println!(
        "The three values of array three_a: {}, {}, {}",
        three_a[0], three_a[1], three_a[2]
    );
}
