use crate::isi::utils::utils::print_compile_error;
use crate::{App, isi::ast::ast::IsiToken};

pub fn scan(app: &App) -> Vec<IsiToken> {
    let mut tokens: Vec<IsiToken> = Vec::new();
    let mut chars = app.content.chars().peekable();
    println!("File content: \n{}", app.content);

    while let Some(&c) = chars.peek() {
        match c {
            '0'..='9' => {
                let mut full_number = String::new();
                while let Some(&d) = chars.peek() {
                    if d.is_ascii_digit() {
                        full_number.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let full_number_as_number: i64 = full_number.parse().unwrap();
                tokens.push(IsiToken::INTEGER(full_number_as_number));
            }
            'a'..='z' => {
                let mut full_str = String::new();
                while let Some(&d) = chars.peek() {
                    if d.is_ascii_alphabetic() {
                        full_str.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(IsiToken::STRING(full_str))
            }
            '-' => {
                tokens.push(IsiToken::MINUS);
                chars.next();

                if chars.peek().unwrap() == &'>' {
                    tokens.pop();
                    tokens.push(IsiToken::ARROW);
                    chars.next();
                }
            }
            '>' => {
                tokens.push(IsiToken::RARROW);
                chars.next();
            }
            '<' => {
                tokens.push(IsiToken::LARROW);
                chars.next();
            }
            _ => {
                if c.is_whitespace() || c == '\r' {
                    chars.next();
                } else if c == '\n' {
                    // inc line count here
                    chars.next();
                } else {
                    print_compile_error(format!("Unknown token: `{}`", c));
                }
            }
        }
    }

    return tokens;
}
