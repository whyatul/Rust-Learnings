use std::io;
use rand::Rng;//Rng trait defines methods that random number generator is implemented
use std::cmp::Ordering; //Ordering is an enum that represents the result of a comparison,  has three variants -- Less, Equal, Greater
//:: - the path separator, used to access something that belongs to a type or module
fn main() {
    println!("----------Guess the Number----------");
    println!("Welcome to the Number Guessing Game");
    println!();
    println!("Please enter your name");
    let mut name = String::new();//new() - an associated function (like a static method) that creates and returns a new empty instance
    io::stdin().read_line(&mut name).expect("Failed to read");
    let name = name.trim();
    println!("Please input your Guess:");

    let secret_number = rand::rng().random_range(1..= 100);//..= means inclusive range
    
    loop {
        let mut guess = String::new();

        //stdin function allow us to handle user input
        io::stdin()
            .read_line(&mut guess) //read line method
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

        println!("The number gussed by {name}: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You win! {guess} is correct!");
                println!("The secret number was {secret_number}");
                break;
            }
            Ordering::Less => {
                let diff = guess.abs_diff(secret_number);//abs_diff() returns the absolute difference between two unsigned integers without any risk of overflow.
                let hint = match diff {
                    1..=3 => "You are very close!",
                    4..=10 => "You are close!",
                    11..=20 => "You are far away!",
                    _ => "You are very far away!",
                };
                println!("{hint}");
            }
            Ordering::Greater => {
                let diff = guess.abs_diff(secret_number);
                let hint = match diff {
                    1..=3 => "You are very close!",
                    4..=10 => "You are close!",
                    11..=20 => "You are far away!",
                    _ => "You are very far away!",
                };
                println!("{hint}");
            }
        }
    }
}
