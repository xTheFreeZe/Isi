use std::env;

use crate::isi::scanner;
pub mod isi;

fn main() {
    let args: Vec<String> = env::args().collect();

    for arg in args {
        println!("{}", arg)
    }
    scanner::scan();
}
