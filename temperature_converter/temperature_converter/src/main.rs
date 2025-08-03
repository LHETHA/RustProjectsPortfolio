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

        let user_selection: i64 = user_selection.trim().parse().expect("Error. Could not read value.");

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