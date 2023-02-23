use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() -> Result<(), ()> {
    let rand_number = rand::thread_rng().gen_range(0..=100);
    // println!("secret number: {rand_number}");

    loop {
        println!("Guess the number!\nPlease input your guess");
        print!(">>> ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(elem) => elem,
            Err(_) => {
                println!("Cannot parse into integer type");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&rand_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    }
    Ok(())
}
