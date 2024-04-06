use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        // This could induce an Error with an empty String reference being passed in normal
        // circumstances
        .read_line(&mut guess)
        .expect("Failed to read line.");

    println!("You guessed: {guess}");

    // `guess` needs to be converted to an Integer type
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
