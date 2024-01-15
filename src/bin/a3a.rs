fn display_message(switch: bool) {
    if switch {
        println!("Hello!");
    } else {
        println!("Goodbye :(");
    }
}
fn main() {
    let switch = false;
    display_message(switch);
}
