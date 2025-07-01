use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess Who!");

    let secret_num = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Input thy guess, lest thou be left in the dark!");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Thy input must be a number!");
                continue;
            }
        };

        println!("Thy guess: {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Thy guess is too small!"),
            Ordering::Greater => println!("Thy guess is too big"),
            Ordering::Equal => {
                println!("Thou hast guessed correctly!");
                break;
            }
        }
    }
}
