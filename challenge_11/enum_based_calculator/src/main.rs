use std::io;

fn menu() {
    println!("\n*************************************************\n");
    println!("Enum Calculator");
    println!("\n*************************************************\n");

    println!("Calculator Menu");
    println!("1. Add");
    println!("2. Subtract");
    println!("3. Multiply");
    println!("4. Divide");
    println!("5. Power");
    println!("6. Square Root");
    println!("7. Exit");
}

#[derive(Debug)]
enum Operations {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    SquareRoot,
}

fn main() {
    // println!("Hello, world!");
    menu();
    let choice: f64 = collect_input("Make a Choice [1 - 7]");

    let operation = match choice as u8 {
        1 => Operations::Add,
        2 => Operations::Subtract,
        3 => Operations::Multiply,
        4 => Operations::Divide,
        5 => Operations::Power,
        6 => Operations::SquareRoot,
        7 => std::process::exit(0),
        _ => {
            println!("Invalid choice");
            std::process::exit(1);
        }
    };

    match operation {
        Operations::Add => {
            let a = collect_input("Give first number");
            let b = collect_input("Give second number");
            println!("Sum: {}", a + b)
        }
        Operations::Subtract => {
            let a = collect_input("Give first number");
            let b = collect_input("Give second number");
            println!("Result: {}", a - b)
        }
        Operations::Multiply => {
            let a = collect_input("Give first number");
            let b = collect_input("Give second number");
            println!("Result: {}", a * b)
        }
        Operations::Power => {
            let a = collect_input("Give number");
            let b = collect_input("Give exponent");
            println!("Result: {}", a.powf(b))
        }
        Operations::SquareRoot => {
            let a = collect_input("Give number");
            println!("Square root: {}", a.sqrt())
        }
        Operations::Divide => loop {
            let a = collect_input("Give Numerator");
            let b = collect_input("Give Denominator");
            if b == 0.0 {
                println!("Cannot divide by Zero(0)");
                continue;
            } else {
                println!("Result: {}", a / b);
                break;
            }
        },
    }
}

fn collect_input(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");
        match input.trim().parse::<f64>() {
            Ok(val) => return val,
            Err(_) => println!("Invalid input. Please enter a number."),
        }
    }
}
