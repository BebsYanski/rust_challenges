use std::io::{self, Write};
fn main() {
    println!("Calculator Program");

    loop {
        // Menu
        println!(
            "\n********************************** Operation Menu **********************************\n"
        );
        println!("1. For Addition (+)");
        println!("2. For Subtraction (-)");
        println!("3. For Multiplication (X)");
        println!("4. For Division (/)");
        println!("5. To Exit");
        println!(
            "\n************************************************************************************\n"
        );

        let choice = match get_input("Give choice from the menu:") {
            Ok(value) => value,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };

        if choice as u8 > 4 {
            // dbg!("Choice should be between 1 and 5");
            break;
        }

        // Get numbers
        let first_number = match get_input("Give first number") {
            Ok(value) => value,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        let second_number = match get_input("Give second number") {
            Ok(value) => value,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        // Pattern matching and operations
        match choice as u8 {
            1 => println!(
                "{} + {} = {}",
                first_number,
                second_number,
                first_number + second_number
            ),
            2 => println!(
                "{} - {} = {}",
                first_number,
                second_number,
                first_number - second_number
            ),
            3 => println!(
                "{} X {} = {}",
                first_number,
                second_number,
                first_number * second_number
            ),
            4 => println!(
                "{} / {} = {:.2}",
                first_number,
                second_number,
                first_number / second_number
            ),
            _ => std::process::exit(1),
        }
    }
}

fn get_input(prompt: &str) -> Result<f64, &str> {
    print!("{} ", prompt);
    io::stdout().flush().unwrap();

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    match choice.trim().parse::<f64>() {
        Ok(value) => Ok(value),
        Err(_) => Err("Choice must be a valid integer (with or without a decimal place)"),
    }
}
