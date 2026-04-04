use std::{io, process};
fn main() {
    println!(" Multiplication Table Generator");

    let mut user_input = String::new();
    let mut number_range = String::new();

    println!("Give number to use for the multiplication table:");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read user input");

    let user_input = user_input
        .trim()
        .parse::<i32>()
        .expect("Input should be a string");

    println!("\nGive number to print multiplication table up to:");

    // Collect range
    io::stdin()
        .read_line(&mut number_range)
        .expect("Failed to read user input");

    let number_range = number_range
        .trim()
        .parse::<i32>()
        .expect("Input should be a string");

    if number_range <= 0 {
        println!("The range should be greater than 0");
        process::abort();
    }

    for i in 1..=number_range {
        println!("{} X {} = {}", i, user_input, i * user_input);
    }
}
