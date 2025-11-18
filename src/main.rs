use crate::isi::ast::ast::App;
use crate::isi::parser::parser::parse;
use crate::isi::scanner::scanner::scan;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub mod isi;

fn main() {
    let mut app = App {
        file_name: String::from(""),
        file_dir: String::from(""),
        content: String::from(""),
        line_count: 1,
        column_count: 0,
        tokens: Vec::new(),
        nodes: Vec::new(),
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
    println!("File name: {}", app.file_name);

    let mut file = File::open(&file_path);
    let mut file_buffer = String::new();

    let bytes_read = match &mut file {
        Ok(f) => f.read_to_string(&mut file_buffer).unwrap(),
        Err(_) => {
            println!(
                "Could not open file: {} with path: {}",
                file_name,
                file_path.display()
            );
            return;
        }
    };

    if bytes_read == 0 {
        println!("Nothing to do > File is empty!");
        return;
    }

    app.content = file_buffer;
    app.tokens = scan(&mut app);
    app.nodes = parse(&mut app);
}
