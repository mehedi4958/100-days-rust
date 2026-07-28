use std::io;

fn main() {
    println!("🧮 Simple Calculator");
    println!("Available Operations: +, -, *, /");
    println!("Enter you expression (e.g. 5 + 3):");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the input");

    let tokens: Vec<&str> = input.trim().split_whitespace().collect();

    if tokens.len() != 3 {
        println!("❌ Invalid input. Please follow the format: number<space>operator<space>number");
        return;
    }

    let num1: f64 = match tokens[0].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("❌ The first number is invalid.");
            return;
        }
    };

    let operator = tokens[1];

    let num2: f64 = match tokens[2].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("❌ The second number is invalid.");
            return;
        }
    };

    let result = match operator {
        "+" => add(num1, num2),
        "-" => subtract(num1, num2),
        "*" => multiply(num1, num2),
        "/" => divide(num1, num2),
        _ => {
            println!("❌ Invalid operator. Use +, -, *, /");
            return;
        }
    };

    println!("✅ Result: {:2}", result);
}

fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        println!("❌ Division by zero is not allowed.");
        std::process::exit(1);
    }
    a / b
}
