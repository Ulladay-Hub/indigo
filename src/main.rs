// Import the necessary modules
use clap::Parser;
use std::fs;
use std::env;
use eframe::{egui, epi};

// Import the Commands enum from the commands module
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

// Define your GUI application
struct MyApp {
    input: String,
    output: String,
}

impl epi::App for MyApp {
    fn name(&self) -> &str {
        "Indigo Terminal"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Indigo Terminal");

            ui.horizontal(|ui| {
                ui.label("root@indigo.exe $");
                ui.text_edit_singleline(&mut self.input);
                if ui.button("Execute").clicked() {
                    self.process_command();
                }
            });

            ui.separator();
            ui.label("Output:");
            ui.label(&self.output);
        });
    }
}

impl MyApp {
    fn process_command(&mut self) {
        // Parse the input command and execute it
        let args = self.input.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();
        let cli = Cli::parse_from(args);
        match cli.command {
            Commands::Print { option } => {
                self.output = match option {
                    Some(name) => format!("Hello, {}!", name),
                    None => "Hello, world!".to_string(),
                };
            }
            Commands::Help { .. }=> {
                self.output = format!("Welcome to Indigo Terminal!\n\n\
                Available commands:\n\
                print: Print a message\n\
                add: Add two numbers\n\
                subtract: Subtract two numbers\n\
                multiply: Multiply two numbers\n\
                divide: Divide two numbers\n\
                modulus: Calculate the modulus of two numbers\n\
                power: Calculate the power of a number\n\
                drv: Change directory and list its contents\n\
                help: Display this help message\n\n\
                Usage:\n\
                print: indigo print -o [message]\n\
                add: indigo add -a [number1] -b [number2]\n\
                subtract: indigo subtract -a [number1] -b [number2]\n\
                multiply: indigo multiply -a [number1] -b [number2]\n\
                divide: indigo divide -a [number1] -b [number2]\n\
                modulus: indigo modulus -a [number1] -b [number2]\n\
                power: indigo power -a [number1] -b [number2]\n\
                drv: indigo drv [directory_path]");
            }
            Commands::Add { option1, option2 } => {
                self.output = match (option1, option2) {
                    (Some(number1), Some(number2)) => format!("{} + {} = {}", number1, number2, number1 + number2),
                    _ => "Please provide two numbers to add".to_string(),
                };
            }
            Commands::Subtract { option1, option2 } => {
                self.output = match (option1, option2) {
                    (Some(number1), Some(number2)) => format!("{} - {} = {}", number1, number2, number1 - number2),
                    _ => "Please provide two numbers to subtract".to_string(),
                };
            }
            Commands::Multiply { option1, option2 } => {
                self.output = match (option1, option2) {
                    (Some(number1), Some(number2)) => format!("{} * {} = {}", number1, number2, number1 * number2),
                    _ => "Please provide two numbers to multiply".to_string(),
                };
            }
            Commands::Divide { option1, option2 } => {
                self.output = match (option1, option2) {
                    (Some(number1), Some(number2)) => {
                        if number2 != 0 {
                            format!("{} / {} = {}", number1, number2, number1 / number2)
                        } else {
                            "Error: Division by zero is not allowed".to_string()
                        }
                    }
                    _ => "Please provide two numbers to divide".to_string(),
                };
            }
            Commands::Modulus { option1, option2 } => {
                self.output = match (option1, option2) {
                    (Some(number1), Some(number2)) => {
                        if number2 != 0 {
                            format!("{} % {} = {}", number1, number2, number1 % number2)
                        } else {
                            "Error: Modulus by zero is not allowed".to_string()
                        }
                    }
                    _ => "Please provide two numbers for modulus operation".to_string(),
                };
            }
            Commands::Power { option1, option2 } => {
                self.output = match (option1, option2) {
                    (Some(number1), Some(number2)) => format!("{} ^ {} = {}", number1, number2, number1.pow(number2 as u32)),
                    _ => "Please provide two numbers for power operation".to_string(),
                };
            }
            Commands::Drv { path } => {
                self.output = match path {
                    Some(dir_path) => {
                        if let Err(e) = env::set_current_dir(&dir_path) {
                            format!("Error: Failed to change directory: {}", e)
                        } else {
                            match fs::read_dir(".") {
                                Ok(entries) => {
                                    let mut contents = String::new();
                                    for entry in entries {
                                        let entry = entry.unwrap();
                                        let path = entry.path();
                                        contents.push_str(&format!("{}\n", path.display()));
                                    }
                                    contents
                                }
                                Err(e) => format!("Error: Failed to read directory contents: {}", e),
                            }
                        }
                    }
                    None => "Please provide a directory path".to_string(),
                };
            }
        };
    }
}

// Main function to run the application
fn main() {
    let app = MyApp {
        input: String::new(),
        output: String::new(),
    };
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
