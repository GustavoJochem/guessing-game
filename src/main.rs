use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut number_of_tries = 0;

    loop {
        println!("\nPlease input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("\nFailed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("\nYou guessed: {}", guess);

        number_of_tries += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("\nToo small!"),
            Ordering::Greater => println!("\nToo big!"),
            Ordering::Equal => {
                println!("\nYou win! Number of tries: {}", number_of_tries);
                break;
            }
        }
    }
}
