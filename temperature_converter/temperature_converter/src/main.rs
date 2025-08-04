use std::io;

fn main() {
    println!("Please select your conversion method!");
    println!("[0] for Fahrenheit --> Celsius");
    println!("[1] for Celsius --> Fahrenheit");

    loop {
        let mut user_selection: String = String::new();

        io::stdin()
            .read_line(&mut user_selection)
            .expect("Error: Cannot read line.");

        println!("Selected conversion: {user_selection}");

        let user_selection: i64 = match user_selection.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if user_selection == 0 {
            fahrenheit_to_celsius();
            break;
        }
        
        else if user_selection == 1 {
            celsius_to_fahrenheit();
            break;
        }

        else {
            println!("Wrong input!");
            break;
        }
    }
}

// Conversion Fahrenheit to Celsius:
// °C = (°F - 32) * 5/9
fn fahrenheit_to_celsius() {
    println!("Please enter your Temperature in Fahrenheit: ");
    let mut user_input: String = String::new();
    
    let user_input = loop {
        io::stdin()
            .read_line(&mut user_input)
            .expect("Error: Failed to read line.");

        println!("Converting!");
        
        match user_input.trim().parse::<f64>() {
            Ok(num) => break num,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        } 
    };

    let converted_temperature: f64 = (user_input - 32.0) * 5.0/9.0;

    println!("Your converted temperature is: {converted_temperature} °C");
}

// Conversion Celsius to Fahrenheit:
// °F = (°C + 32) * 9/5
fn celsius_to_fahrenheit() {
    println!("Please enter your temperature in Celsius: ");
    let mut user_input: String = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Error. Could not read line.");

    println!("Converting!");
    
    let user_input: f64 = match user_input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };

    let converted_temperature: f64 = (user_input * 9.0/5.0) + 32.0;

    println!("Your converted temperature is: {converted_temperature} °F")
}