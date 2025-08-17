use std::cmp::Ordering;
use::std::io;
use::rand::Rng;//Rng trait defines methods that random number generator is implemented

fn main() {
    println!("----------Guess the Number----------");
    println!();
    println!("Welcome to the Number Guessing Game");
    println!();
    println!("Please input your Guess:");
    
    let secret_number = rand::rng().random_range(1..=100);
    
    loop {
        let mut guess = String::new();
        
        //stdin function allow us to handle user input
        io::stdin()
            .read_line(&mut guess)//read line method
            .expect("Failed to read");

        //shadowing(convert a value from one type to another type)
        //The trim method on a String instance will eliminate any whitespace at the beginning and end
        //The parse method on strings converts a string to another type -  only work on characters
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid number!");
                continue;
            }
        };
        
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number){
            Ordering::Less => println!("{guess} is too small!"),
            Ordering::Equal => {
                println!("You win! {guess} is correct!");
                println!("The secret number was {secret_number}");
                break;
            },
            Ordering::Greater=> println!("{guess} is too big!"),
        }
    }
}