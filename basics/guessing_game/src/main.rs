extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("Guess.");
    
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    loop{
        let mut guess = String::new(); // a place to store the user input
        // it created a mutable variable that is currently bound to a new,
        // empty instance of a string.
        // ::new line indicates that new is an associated function of the string type
        // some languages call it a static method

        io::stdin().read_line(&mut guess) 
            .expect("Failed to read line"); //handling potential failure with result type
        // the studin function returns an instance of std::io::Stdin
        
        // It handles the standard input for your terminal
        // readline, with argument: &mut guess
        
        // the job of read_line is to take whatever the user types into standard input
        // and place that into a string.
        
        // & indicates that this argument is a reference, which gives you a way to
        // let multiple parts of your code access one piece of data without needing to copy that
        // data into memory multiple times

        // you need to write &mut guess and not &guess to make it mutable

        // handling potential failures
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        //    .expect("Type a number!"); // parse() method converts a string to another type


        println!("Your guess: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
