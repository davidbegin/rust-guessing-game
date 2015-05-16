extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("Enter your guess.");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .ok()
        .expect("Failed to read input");

    println!("Woah I have the guess! {}", guess);

    println!("I can print a random number! {}", secret_number);
}
