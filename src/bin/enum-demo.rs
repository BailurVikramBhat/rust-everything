enum Direction {
    Left,
    Right,
}
fn main() {
    let go = Direction::Left;
    match go {
        Direction::Left => {
            println!("Ah nice, going left.");
            println!("Maybe right misses you?")
        }
        Direction::Right => println!("So you are right it seems"),
    }
    let do_not_go = Direction::Right;
    match do_not_go {
        Direction::Left => println!("Why not going left?"),
        Direction::Right => println!("Why not going right?"),
    }
}
