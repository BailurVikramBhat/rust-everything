fn main() {
    let my_name = "Vikram";
    match my_name {
        "Bob" => println!("Hi Bob!"),
        "Alice" => println!("Are you really Alice?!"),
        "Ross" => println!("You are the Bob Ross of programming!"),
        _ => println!("Ah you are generic..."),
    };
}
