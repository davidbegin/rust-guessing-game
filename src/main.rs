use std::io;

fn main() {
    println!("Guess the number!");
    println!("Enter your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .ok()
        .expect("Failed to read input");

    println!("Woah I have the guess! {}", guess);
}
