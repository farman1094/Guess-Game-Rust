use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // converting the number into the uint
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                // Only number is supported
                println!("Please input number only.");
                continue; // continue the loop
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You Won");
                break; // exit the game when guessed right
            }
        }
    }
}
