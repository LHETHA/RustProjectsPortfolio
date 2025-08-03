use std::io;

fn main() {
    fahrenheit_to_celsius();
}

// Conversion Fahrenheit to Celsius:
// °C = (°F - 32) * 5/9
fn fahrenheit_to_celsius() {
    println!("Please enter your Temperature in Fahrenheit: ");
    let mut user_input: String = String::new();

    println!("Converting!");

    io::stdin()
        .read_line(&mut user_input)
        .expect("Error: Failed to read line.");
    
    let user_input: i64 = user_input.trim().parse().expect("Incorrect value.");

    let converted_temperature: i64 = (user_input - 32) * 5/9;

    println!("Your converted temperature is: {converted_temperature} °C");
}