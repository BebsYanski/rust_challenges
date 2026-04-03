use rand::prelude::*;
use std::io::{self, Write};

fn main() {
    let mut rng = rand::rng();

    println!("Number Guessing Game");
    let computer_guess = rng.random_range(1..=100);
    let mut input = String::new();

    loop {
        println!("The computer has a random number between 1 and 100 inclusive.");
        print!("Try to guess the number:\t");
        io::stdout().flush().unwrap();
        // input.clear();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<i32>() {
            Ok(guess) => {
                if guess == computer_guess {
                    println!("You guessed the number!");
                    break;
                } else if guess < computer_guess {
                    println!("Too low");
                } else {
                    println!("Too high");
                }
                input.clear();
            }
            Err(_) => println!("Please enter a valid number"),
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
