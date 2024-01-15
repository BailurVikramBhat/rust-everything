use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Let's play 'GUESS A NUMBER!");
    println!("...");
    println!("generating secret number please wait...");
    println!("...");
    let secret_number = rand::thread_rng().gen_range(0..=100);
    let mut quit_badly: u32 = 0;
    println!("secret number generated successfully. ");
    loop {
        println!("Your guess: ");
        let mut response = String::new();
        io::stdin()
            .read_line(&mut response)
            .expect("How could you? :(");
        let response: u32 = match response.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                quit_badly = quit_badly + 1;
                if quit_badly == 3 {
                    println!("You poor thing gave up... :(");
                    println!("The answer was {secret_number}");
                    break;
                }
                continue;
            }
        };
        match response.cmp(&secret_number) {
            Ordering::Greater => println!("Too big..."),
            Ordering::Equal => {
                println!("You WIN!!!!");
                break;
            }
            Ordering::Less => println!("Too less..."),
        };
    }
}
