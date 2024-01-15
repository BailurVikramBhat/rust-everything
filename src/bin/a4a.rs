fn main() {
    // decision making using a match expression
    let my_switch = true;
    match my_switch {
        true => println!("Ah I see you are a man of culture as well..."),
        false => println!("Ah shit here we go again :|"),
    };
    match my_switch {
        _ => println!("If it works it works."),
    };
}
