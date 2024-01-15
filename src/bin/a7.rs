use core::fmt;
use std::fmt::write;

enum Color {
    Red,
    Blue,
    Green,
    Yellow,
    White,
    Black,
}
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Red => write!(f, "Red"),
            Color::Blue => write!(f, "Blue"),
            Color::Green => write!(f, "Green"),
            Color::Yellow => write!(f, "Yellow"),
            Color::White => write!(f, "White"),
            Color::Black => write!(f, "Black"),
        }
    }
}

fn main() {
    let color = Color::Blue;
    match color {
        Color::Red => println!("{:?} is a great choice!", color.to_string()),
        Color::Black => println!("I really like your color taste"),
        Color::Blue => println!("You sure?"),
        _ => println!(
            "Weirdo! who in the world has {:?} as fav color!",
            color.to_string()
        ),
    }
}
