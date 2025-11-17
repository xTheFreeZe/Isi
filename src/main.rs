use crate::isi::ast::ast::App;
use std::env;
use std::path::Path;

pub mod isi;

fn main() {
    let mut app = App {
        file_name: String::from(""),
        file_dir: String::from(""),
        content: String::from(""),
    };

    let mut file_name = String::from("");

    for arg in env::args() {
        if arg.contains(".isi") {
            file_name = arg
        }
    }

    let file_exists = Path::new(&file_name).exists();

    if !file_exists {
        println!("File does not exist");
        return;
    }

    app.file_name = String::from(&file_name);

    let file_path = Path::new(&file_name);
    let mut dir = env::current_dir().unwrap();

    if let Some(parent) = file_path.parent() {
        if !parent.as_os_str().is_empty() {
            dir.push(parent);
        }
    }

    app.file_dir = dir.to_string_lossy().into_owned();

    println!("File path: {}", app.file_dir);
}
