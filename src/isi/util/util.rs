use colored::Colorize;
use std::process::exit;

pub fn print_compile_error(error: &str) {
    let message = error.red();
    println!("\nIsi compile error: \n{}", message);
    exit(1);
}
