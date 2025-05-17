mod operations;

use operations::{add, subtract, multiply, divide};
use std::io::{self, Write};

fn main() {
    println!("Simple Rust Calculator");

    let num1 = read_number("Enter the first number: ");
    let operator = read_operator("Enter an operator (+, -, *, /): ");
    let num2 = read_number("Enter the second number: ");

    let result = match operator.as_str() {
        "+" => Ok(add(num1, num2)),
        "-" => Ok(subtract(num1, num2)),
        "*" => Ok(multiply(num1, num2)),
        "/" => divide(num1, num2),
        _ => Err("Invalid operator".to_string()),
    };

    match result {
        Ok(value) => println!("Result: {}", value),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn read_number(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            if let Ok(num) = input.trim().parse::<f64>() {
                return num;
            } else {
                println!("Invalid number. Try again.");
            }
        }
    }
}

fn read_operator(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
