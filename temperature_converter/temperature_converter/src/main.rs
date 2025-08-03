use std::io;

fn main() {
    fahrenheit_to_celsius();
    celsius_to_fahrenheit();
}

// Conversion Fahrenheit to Celsius:
// °C = (°F - 32) * 5/9
fn fahrenheit_to_celsius() {
    println!("Please enter your Temperature in Fahrenheit: ");
    let mut user_input: String = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Error: Failed to read line.");

    println!("Converting!");
    
    let user_input: i64 = user_input.trim().parse().expect("Incorrect value.");

    let converted_temperature: i64 = (user_input - 32) * 5/9;

    println!("Your converted temperature is: {converted_temperature} °C");
}

fn celsius_to_fahrenheit() {
    println!("Please enter your temperature in Celsius: ");
    let mut user_input: String = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Error. Could not read line.");

    println!("Converting!");
    
    let user_input: i64 = user_input.trim().parse().expect("Error. Cannot read value.");

    let converted_temperature: i64 = (user_input * 9/5) + 32;

    println!("Your converted temperature is: {converted_temperature} °F")
}