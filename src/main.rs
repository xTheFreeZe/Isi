use execute::Execute;

use crate::isi::ast::ast::App;
use crate::isi::generator::generator::generator;
use crate::isi::parser::parser::parse;
use crate::isi::scanner::scanner::scan;
use crate::isi::util::util::print_compile_error;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, exit};
use std::sync::Arc;

pub mod isi;

fn main() {
    let mut app = App {
        file_name: Arc::from(""),
        file_dir: Arc::from(""),
        content: Arc::from(""),
        line_count: 1,
        column_count: 1,
        index: 0,
        tokens: Vec::new(),
        nodes: Vec::new(),
        current_var_str: String::new(),
        function_table: HashMap::new(),
        variable_table: HashMap::new(),
        generated_code: String::new(),
    };
    let file_name: String = env::args().filter(|arg| arg.ends_with(".isi")).collect();

    if file_name.is_empty() {
        print_compile_error("No input files");
    }

    let file_exists = Path::new(&file_name).exists();

    if !file_exists {
        print_compile_error("File does not exist");
    }

    app.file_name = Arc::from(file_name);

    let file_path = Path::new(app.file_name.as_ref());
    let mut dir = env::current_dir().unwrap();

    if let Some(parent) = file_path.parent() {
        if !parent.as_os_str().is_empty() {
            dir.push(parent);
        }
    }

    app.file_dir = Arc::from(dir.to_string_lossy());

    let mut file = File::open(&file_path);
    let mut file_buffer = String::new();

    // Add the std functions first
    //  This is awkward and needs to be changed. Maybe add the ast nodes of the parsed file instead?
    let std_path = if env::consts::OS == "windows" {
        "std\\core.isi"
    } else {
        "std/core.isi"
    };
    let mut std_file = File::open(&std_path);
    match &mut std_file {
        Ok(f) => f.read_to_string(&mut file_buffer).unwrap(),
        Err(_) => {
            print_compile_error(&format!("Standard Lib not found under: {}", std_path));
            exit(1);
        }
    };

    let bytes_read = match &mut file {
        Ok(f) => f.read_to_string(&mut file_buffer).unwrap(),
        Err(_) => {
            print_compile_error(&format!(
                "Could not open file: {} with path: {}",
                app.file_name.clone(),
                file_path.display()
            ));
            exit(1);
        }
    };

    if bytes_read == 0 {
        print_compile_error("File is empty > Nothing to do");
    }

    app.content = Arc::from(file_buffer);
    app.tokens = scan(&mut app);
    parse(&mut app);
    // Reset the index so the generator can use it
    app.index = 0;
    generator(&mut app);
    // for node in &app.nodes {
    //     println!("{:#?}", node);
    // }
    // let func = app.get_function_from_map("print");
    // println!("{func:?}");

    // C File Stuff
    let c_path = format!("{}.c", app.file_name.as_ref());
    // let exe_path_result: Option<(&str, &str)> = if env::consts::OS == "windows" {
    //     let exe_path_after_slash = c_path.split_once("/");
    //     let mut wow: Option<(&str, &str)>;
    //     if let Some(path) = exe_path_after_slash {
    //         wow = path.1.split_once(".");
    //     } else {
    //         eprintln!("Was unable to generate windows executable name");
    //         exit(1);
    //     }
    //     wow
    // } else {
    //     c_path.split_once(".")
    // };
    // if exe_path_result.is_none() {
    //     eprintln!("Was unable to split file while generating executable name");
    //     exit(1);
    // }
    let stem = Path::new(&c_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_else(|| {
            eprintln!("Was unable to generate executable name");
            std::process::exit(1);
        });

    let exe_name = if cfg!(windows) {
        format!("{stem}.exe")
    } else {
        stem.to_string().replace(".isi", "")
    };
    let c_file = File::create(&c_path);
    if let Ok(mut file) = c_file {
        let write_result = file.write(app.generated_code.as_bytes());
        if write_result.is_err() {
            eprintln!("Was unable to write to file");
            exit(1);
        }
    }

    println!("{exe_name}");

    let mut command = Command::new("gcc");
    command.arg(&c_path);
    command.arg("-o");
    command.arg(&exe_name);

    if command.execute_check_exit_status_code(0).is_err() {
        eprintln!("Was unable to compile generated c file using gcc");
        exit(1);
    }

    Command::new(format!("./{}", exe_name))
        .status()
        .expect("Can't run executable");
}
