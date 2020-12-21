use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess a number!");
    //generate a random number
    let secret_num = rand::thread_rng().gen_range(1,101);
    loop{
        println!("Please input your guess.");
        //create a mutable string variable to hold the guess
        let mut guess = String::new();
        //store user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
        //convert guess from string to u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        //print the guess to console
        println!("You guessed: {}", guess);

        //compare guess with generated number
        match guess.cmp(&secret_num){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Congrats! You guessed the right number.");
                break;
            }
        }
    }
}
