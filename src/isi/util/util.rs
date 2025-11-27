use std::process::exit;

use colored::Colorize;

pub fn print_compile_error(error: String) {
    let message = error.red();
    println!("\nIsi compile error: \n{}", message);
    exit(1);
}
