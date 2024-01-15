fn main() {
    println!("Casually starting the program: ");
    let mut looper = 1;

    loop {
        println!("{:?}", looper);
        if looper == 4 {
            break;
        }
        looper = looper + 1;
    }
    println!("Done-!");
}
