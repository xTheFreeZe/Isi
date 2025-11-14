use crate::isi::ast::App;
use std::env;
use std::path::Path;

pub mod isi;

fn main() {
    let mut app = App {
        file_name: String::from(""),
        path_to_file: String::from(""),
        content: String::from(""),
    };

    let mut file_name = String::from("");

    for arg in env::args() {
        if arg.contains(".isi") {
            file_name = arg
        }
    }

    println!("File name: {}", &file_name);

    let file_exists = Path::new(&file_name).exists();

    if !file_exists {
        println!("File does not exist");
        return;
    }

    println!("File does in fact exist!");
    app.file_name = String::from(file_name);
}
