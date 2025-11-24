use std::process::exit;

use colored::Colorize;

/// Throws the given error and exits the program with code `1`
pub fn print_compile_error(error: String) {
    let message = error.red();
    println!("\nIsi compile error: \n{}", message);
    exit(1);
}
