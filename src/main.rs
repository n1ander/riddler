use std::io;
use rand::Rng;

fn main() {
    println!("Guess a number!");
    //generate a random number
    let secret_num = rand::thread_rng().gen_range(1,101);
    println!("The secret number is: {}", secret_num);
    println!("Please input your guess.");
    //create a mutable string variable to hold the guess
    let mut guess = String::new();
    //store user input
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");
    //print the guess to console
    println!("You guessed: {}", guess)
}
