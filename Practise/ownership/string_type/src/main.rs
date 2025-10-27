fn main() {
    let mut string_variable = String::from("Hello");
    string_variable.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", string_variable); // this will print `hello, world!`

    // Memory issue double free error
    //     let s1 = String::from("hello");
    //     let s2 = s1;
    //     println!("{}", s1);

    //Deep cloning
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
}
