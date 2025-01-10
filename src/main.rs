use std::io;

fn main() {
    println!("Welcome to the temperature converter!");

    loop {
        let user_choice = get_user_input("To convert Fahrenheit to Celsius, input 1. To convert Celsius to Fahrenheit, input 2.");

        if user_choice == 1.0 {
            let fahr = get_user_input("Fahrenheit degrees: ");

            let cel = fahrenheit_to_celcius(fahr);
            println!("{} degrees Fahrenheit is {} degrees Celsius", fahr, cel);
        } else if user_choice == 2.0 {
            let cel = get_user_input("Celsius degrees: ");

            let fahr = celcius_to_fahrenheit(cel);
            println!("{} degrees Celsius is {} degrees Fahrenheit", cel, fahr);
        } else {
            println!("Invalid input. Please enter 1 or 2.");
        }

        let user_choice_end = get_user_input_end("Would you like to convert one more time? (y/n)");
        if user_choice_end == "n" {
            println!("Thank you and goodbye!");
            break;
        }

    }


}

fn fahrenheit_to_celcius(fahr: f32) -> f32 {
    (fahr - 32.0) * 5.0 / 9.0
}

fn celcius_to_fahrenheit(cel: f32) -> f32 {
    cel * 9.0 / 5.0 + 32.0
}

fn get_user_input(prompt: &str) -> f32 {
    loop {
        let mut input = String::new();
        println!("{}", prompt);
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter a number."),
        }
    }
}

fn get_user_input_end(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}