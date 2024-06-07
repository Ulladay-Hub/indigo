// Import the necessary modules
use clap::{Parser, Subcommand};

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

// Parser
#[derive(Subcommand)]
enum Commands {
    Print {
        #[arg(short, long)]
        option: Option<String>,
    },
    Add {
        #[arg(short = 'a', long)]
        option1: Option<i32>,
        #[arg(short = 'b', long)]
        option2: Option<i32>,
    },
    Subtract {
        #[arg(short = 'a', long)]
        option1: Option<i32>,
        #[arg(short = 'b', long)]
        option2: Option<i32>,
    },
    Multiply {
        #[arg(short = 'a', long)]
        option1: Option<i32>,
        #[arg(short = 'b', long)]
        option2: Option<i32>,
    }
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
    }
}
