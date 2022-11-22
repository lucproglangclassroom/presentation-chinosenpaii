use std::io; //library to obtain user input and output it
use rand::Rng; //lets us implement a random number generator
use std::cmp::Ordering; //library that lets us compare

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); //variable that randomly generates a number between 1-100


    loop {
        println!("The secret number is: {secret_number}");

        println!("Please input your guess.");

        let mut guess = String::new(); //mutable variable bound to a new empty instance of a string

        io::stdin() // handle user input
            .read_line(&mut guess) //read user input and append to the variable guess
            .expect("Failed to read line"); //error text

        let guess: u32 = match guess.trim().parse() { //calls guess again, trims whitespace, converts from string to int
            Ok(num) => num, //if number is put in then it will be put into guess
            Err(_) => continue, //anything but a number the loop will continue
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) { //compares guess to the secret number
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
