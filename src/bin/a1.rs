// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal
fn display_name(first_name: &str, last_name: &str) {
    println!("Hello Mr. {first_name:?} {last_name:?}!");
}
fn main() {
    display_name("Vikram", "Bhat");
}
