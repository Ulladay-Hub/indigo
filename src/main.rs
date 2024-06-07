// Import the necessary modules
use clap::Parser;
mod commands;
use commands::Commands;

// Application information
#[derive(Parser)]
#[command(name = "indigo")]
#[command(version = "1.0")]
#[command(author = "sirbradinator <imnotamilkglass@gmail.com>")]
#[command(about = "An advanced terminal program")]

// Command structure [(command) (subcommand)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    // Set the device name
    let device_name = "indigo.exe";

    // Parse command-line arguments
    let cli = Cli::parse();

    // Print prompt
    print!("root@{} $ ", device_name);

    // Process commands
    match cli.command {
        // Print command
        Commands::Print { option } => {
            if let Some(name) = option {
                println!("Hello, {}!", name);
            } else {
                println!("Hello, world!");
            }
        }
        // Add command
        Commands::Add { option1, option2 } => {
            if let (Some(number1), Some(number2)) = (option1, option2) {
                println!("{} + {} = {}", number1, number2, number1 + number2);
            } else {
                println!("Please provide two numbers to add");
            }
        }
        // Subtract command
        Commands::Subtract { option1, option2 } => {
            if let (Some(number1), Some(number2)) = (option1, option2) {
                println!("{} - {} = {}", number1, number2, number1 - number2);
            } else {
                println!("Please provide two numbers to subtract");
            }
        }
        // Multiply command
        Commands::Multiply { option1, option2 } => {
            if let (Some(number1), Some(number2)) = (option1, option2) {
                println!("{} * {} = {}", number1, number2, number1 * number2);
            } else {
                println!("Please provide two numbers to multiply");
            }
        }
        // Divide command
        Commands::Divide { option1, option2 } => {
            if let (Some(number1), Some(number2)) = (option1, option2) {
                if number2 != 0 {
                    println!("{} / {} = {}", number1, number2, number1 / number2);
                } else {
                    println!("Error: Division by zero is not allowed");
                }
            } else {
                println!("Please provide two numbers to divide");
            }
        }
        // Modulus command
        Commands::Modulus { option1, option2 } => {
            if let (Some(number1), Some(number2)) = (option1, option2) {
                if number2 != 0 {
                    println!("{} % {} = {}", number1, number2, number1 % number2);
                } else {
                    println!("Error: Modulus by zero is not allowed");
                }
            } else {
                println!("Please provide two numbers for modulus operation");
            }
        }
        // Power command
        Commands::Power { option1, option2 } => {
            if let (Some(number1), Some(number2)) = (option1, option2) {
                println!("{} ^ {} = {}", number1, number2, number1.pow(number2 as u32));
            } else {
                println!("Please provide two numbers for power operation");
            }
        }
    }
}
