fn main() {
    let some_int = 19;
    match some_int {
        1 => println!("1 matched!"),
        2 => println!("2 matched!!"),
        3 => println!("3 matched!!!"),
        10 => println!("10 matched!"),
        _ => println!("none matched :("),
    }
}
