use std::io;
use std::io::Write;

pub fn convert_it() {
    loop {
        println!("Temperature Conversion:");
        println!("1. Fahrenheit to Celsius");
        println!("2. Celsius to Fahrenheit");
        println!("3. Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim().parse::<u32>() {
            Ok(1) => {
                let fahrenheit = get_input("Enter temperature in Fahrenheit: ");
                let celsius = fahrenheit_to_celsius(fahrenheit);
                println!("{:.2} Fahrenheit is {:.2} Celsius\n", fahrenheit, celsius);
            },
            Ok(2) => {
                let celsius = get_input("Enter temperature in Celsius: ");
                let fahrenheit = celsius_to_fahrenheit(celsius);
                println!("{:.2} Celsius is {:.2} Fahrenheit\n", celsius, fahrenheit);
            },
            Ok(3) => {
                println!("Exiting...");
                break;
            },
            _ => println!("Invalid choice, please try again.\n"),
        }
    }
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn get_input(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<f64>() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input, please enter a number."),
        }
    }
}
