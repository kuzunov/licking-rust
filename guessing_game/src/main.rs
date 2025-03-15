use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    let secret = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret}");

    println!("Guess my ballz!");
    loop {
        println!("GUESS NOW OR STFU FOREVER: ");

        let mut guessed = String::new();

        io::stdin().read_line(&mut guessed).expect("fail");

        let guessed: u32 = match guessed.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guessed.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        println!("My balz ar: {}", guessed);
    }
}
