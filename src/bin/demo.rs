fn main() {
    let age = 15;
    if age >= 21 {
        println!("You are allowed to enter... please proceed.");
    } else {
        println!(
            "Not allowed. You need to wait for {:?} more years.",
            21 - 15
        );
    }
}
