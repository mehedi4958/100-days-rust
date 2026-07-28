use std::io;

fn main() {
    println!("🌡️ Temperature converter");
    println!("1: Celsius to Fahrenheit");
    println!("2: Fahrenheit to Celsius");
    println!("Please select option 1 or 2:");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");

    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("❌ Invalid choise. Please enter 1 or 2.");
            return;
        },
    };

    if choice == 1 {
        celsius_to_fahrenheit();
    }else if choice == 2 {
        fahrenheit_to_celcius();
    }else{
       println!("❌ Invalid choise. Please enter 1 or 2."); 
    }
}

fn celsius_to_fahrenheit() {
    println!("Enter the Temperature in Celsius:");

    let mut temperature = String::new();
    io::stdin().read_line(&mut temperature).expect("Failed to read input");

    let temperature: f64 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("❌ Invalid input. Please enter a valid number.");
            return;
        },
    };

    let fahrenheit = (temperature * 9.0 / 5.0) + 32.0;
    println!("{:.2}°C is {:.2}°F", temperature, fahrenheit);
}

fn fahrenheit_to_celcius() {
    println!("Enter the Temperature in Fahrenheit:");

    let mut temperature = String::new();
    io::stdin().read_line(&mut temperature).expect("Failed to read input");

    let temperature: f64 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("❌ Invalid input. Please enter a valid number.");
            return;
        },
    };

    let celsius = (temperature -32.0) * 5.0 / 9.0;
    println!("{:.2}°F is {:.2}°C", temperature, celsius);
}