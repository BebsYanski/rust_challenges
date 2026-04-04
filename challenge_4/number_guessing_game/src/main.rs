use rand::prelude::*;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    loop {
        let mut rng = rand::rng();
        let mut attempts = 0;
        let attempts_limit = 10;
        let mut guessed_numbers: Vec<i32> = Vec::new();

        println!("\n--------------------------------");
        println!("Number Guessing Game"); // print the title of the game
        let computer_guess = rng.random_range(1..=100);
        let mut input = String::new(); // create a new string to store the input

        while attempts < attempts_limit {
            println!("\n--------------------------------");
            println!("The computer has a random number between 1 and 100 inclusive."); // print the instructions
            print!("Try to guess the number:\t"); // print the prompt
            io::stdout().flush().unwrap(); // flush the output
            // input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input"); // read the input

            match input.trim().parse::<i32>() {
                // parse the input
                Ok(guess) => {
                    if guessed_numbers.contains(&guess) {
                        println!("You have already guessed this number. Try again."); // if the number has already been guessed, print the message and continue the loop
                        continue;
                    }
                    guessed_numbers.push(guess); // add the guess to the vector

                    // if the input is a valid number
                    match guess.cmp(&computer_guess) {
                        // compare the guess with the computer's guess
                        Ordering::Equal => {
                            println!("You guessed the number!"); // if the guess is correct, print the message and break the loop
                            println!("You guessed the number in {} attempts.", attempts); // print the number of attempts
                            break;
                        }
                        Ordering::Less => println!("Too low"), // if the guess is less than the computer's guess, print the message
                        Ordering::Greater => println!("Too high"), // if the guess is greater than the computer's guess, print the message
                    }
                    input.clear(); // clear the input
                    attempts += 1; // increment the attempts
                    if attempts == attempts_limit {
                        println!("You lost! The number was {}.", computer_guess); // if the attempts are over the limit, print the message and break the loop
                        break;
                    }
                    println!("You have {} attempts left.", attempts_limit - attempts); // print the number of attempts left
                }
                Err(_) => println!("Please enter a valid number"),
            }
        }

        println!("You guessed the numbers: {:?}", guessed_numbers); // print the guessed numbers

        // Menu
        let mut choice = String::new();
        println!("Do you wish to Replay? \n1- Yes\n2- No\n");
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim() {
            "1" => continue,
            "2" => break,
            _ => std::process::exit(1),
        }
    }
}

// fn main() {
//     let mut rng = rand::rng();

//     println!("Random die roll: {}", rng.random_range(1..=6));
//     println!("Random UUID: 0x{:X}", rng.random::<u128>());

//     if rng.random() {
//         println!("You got lucky!");
//     }

//     // get some random data:
//     let mut data: [u8; 8] = [0u8; 8];
//     let hello = rand::rng().next_u32();
//     let an_hello = rng.next_u64();
//     let ann_hello = rng.random::<i32>();
//     rand::rng().fill_bytes(&mut data);
//     println!("{:?}", data);
//     println!("{} - {} - {}", hello, an_hello, ann_hello);
// }
