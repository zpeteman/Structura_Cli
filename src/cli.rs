// cli.rs
use colored::*; // Add the `colored` crate for styling output
use std::thread::sleep;
use std::time::Duration;

use figlet_rs::FIGfont;

pub fn display_cli() {
    // Load a FIGfont for ASCII art (Standard font is used here)
    let standard_font = FIGfont::standard().unwrap();
    let ascii_art = standard_font.convert("Structura CLI");

    // Print the ASCII art title
    if let Some(ascii_art) = ascii_art {
        println!("{}", ascii_art);
    }

    // Display version, author, and other details
    println!("Version: 0.6.16");
    println!("Author: Ilyas Zanan");
    println!("--------------------------------------------------");
    println!("Instructions:");
    println!("- it doesn't work in directories which are not in the home directory.");
    println!("- to use it just enter the directory path correctly.");
    println!("--------------------------------------------------");

    // Simulate starting the CLI
    println!("{}", "Starting Structura CLI...\n".green());
    sleep(Duration::from_secs(1));
    println!("{}", "Running...".cyan());
    sleep(Duration::from_secs(2)); 
}

pub fn error() {
    println!("{}","ERROR: This directory doesn't contains any files".red());
}

pub fn done() {
    println!("{}","Done".green());
}
