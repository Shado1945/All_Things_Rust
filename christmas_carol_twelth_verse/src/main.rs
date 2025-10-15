fn main() {
    let verses = [
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings",
        "Four calling birds",
        "Three french hens",
        "Two turtle doves and",
        "A partridge in a pear tree",
    ];

    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    for xmas in 1..=12 {
        println!(
            "\nOn the {} day of Christmas, my true love sent to me",
            days[xmas - 1]
        );
        for verse in &verses[12 - xmas..] {
            println!("{}", verse);
        }
    }
}
