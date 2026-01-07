use colored::Colorize;

use crate::isi::ast::ast::App;
use crate::isi::parser::parser::parse;
use crate::isi::scanner::scanner::scan;
use crate::isi::util::util::print_compile_error;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::exit;

pub mod isi;

fn main() {
    let mut app = App {
        file_name: String::from(""),
        file_dir: String::from(""),
        content: String::from(""),
        line_count: 1,
        column_count: 1,
        index: 0,
        tokens: Vec::new(),
        nodes: Vec::new(),
    };
    let mut file_name = String::from("");

    let args = env::args();

    if args.len() == 0 {
        print_compile_error("No input files");
    }

    for arg in args {
        if arg.contains(".isi") {
            file_name = arg
        }
    }

    if file_name.is_empty() {
        print_compile_error("No input files");
    }

    let file_exists = Path::new(&file_name).exists();

    if !file_exists {
        print_compile_error("File does not exist");
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

    let mut file = File::open(&file_path);
    let mut file_buffer = String::new();

    let bytes_read = match &mut file {
        Ok(f) => f.read_to_string(&mut file_buffer).unwrap(),
        Err(_) => {
            print_compile_error(&format!(
                "Could not open file: {} with path: {}",
                file_name,
                file_path.display()
            ));
            exit(1);
        }
    };

    if bytes_read == 0 {
        print_compile_error("File is empty > Nothing to do");
    }

    app.content = file_buffer;
    app.tokens = scan(&mut app);
    parse(&mut app);

    println!("{}", "Done".bright_blue());

    for node in app.nodes {
        println!("{:?}", node);
    }
}
