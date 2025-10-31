fn main() {
    println!("This program converts farenheit to celsius");
    let far: u32 = 77;
    let cel = conver_far_to_cel(far);
    println!("Degrees in farenheit: {} to celsius: {}", far, cel);
}

fn conver_far_to_cel(f: u32) -> u32 {
    ((f - 32) * 5) / 9
}
