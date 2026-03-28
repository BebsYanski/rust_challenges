use std::io::{self, Write};
fn main() {
    println!("Temperature Converter");

    loop {
        println!(
            "********************************** OPERATION MENU ************************************\n"
        );
        println!("1. To Convert from Celsius to Fahrenheit");
        println!("2. To Convert from Fahrenheit to Celsius");
        println!("3. to Exit");
        println!("**********************************************************************\n");

        let choice = match get_input("Choose an option from the menu:") {
            Some(value) => value,
            None => {
                continue;
            }
        };

        if choice as u8 > 2 {
            break;
        }

        let temp = match get_input("Give temperature for conversion:") {
            Some(value) => value,
            None => {
                continue;
            }
        };

        // Pattern Matching
        match choice as u8 {
            1 => cel_to_fah(temp),
            2 => fah_to_cel(temp),
            3 => {
                println!("Thanks for using this system");
                println!("Bye for now");
                break;
            }
            _ => std::process::abort(),
        }
    }
}

fn get_input(prompt: &str) -> Option<f64> {
    print!("{} ", prompt);
    io::stdout().flush().unwrap();

    let mut choice: String = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    match choice.trim().parse::<f64>() {
        Ok(value) => Some(value),
        Err(_) => {
            println!("Expected an integer (with or without a decimal place)");
            None
        }
    }
}

fn cel_to_fah(temp: f64) {
    let new_temp = (temp * (9f64 / 5f64)) + 32f64;

    println!("Converted: {}°C -> {}°F", temp, new_temp);
}

fn fah_to_cel(temp: f64) {
    let new_temp = (temp - 32f64) * (5f64 / 9f64);

    println!("Converted: {}°F -> {}°C", temp, new_temp);
}
