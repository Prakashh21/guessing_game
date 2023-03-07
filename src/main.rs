use std::io::{self, Read};

fn main() {
    println!("Guess a Number!");

    println!("Please enter your guess number");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    print!("you guessed: {guess}");




}
