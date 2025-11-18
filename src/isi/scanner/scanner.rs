use crate::isi::ast::ast::Token;
use crate::isi::utils::utils::print_compile_error;
use crate::{App, isi::ast::ast::IsiToken};

pub fn scan(app: &mut App) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = app.content.chars().peekable();
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
                tokens.push(Token {
                    t_value: full_number,
                    t_type: IsiToken::INTEGER(full_number_as_number),
                    t_line: app.line_count,
                    t_column: app.column_count,
                });
            }
            'a'..='z' => {
                let mut full_str = String::new();
                while let Some(&d) = chars.peek() {
                    if d.is_ascii_alphanumeric() || d == '_' {
                        full_str.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token {
                    t_value: String::from(&full_str),
                    t_type: IsiToken::STRING(full_str),
                    t_line: app.line_count,
                    t_column: app.column_count,
                });
            }
            '-' => {
                tokens.push(Token {
                    t_value: String::from(c),
                    t_type: IsiToken::MINUS,
                    t_line: app.line_count,
                    t_column: app.column_count,
                });
                chars.next();

                if chars.peek().unwrap() == &'>' {
                    tokens.pop();
                    tokens.push(Token {
                        t_value: String::from("=>"),
                        t_type: IsiToken::ARROW,
                        t_line: app.line_count,
                        t_column: app.column_count,
                    });
                    chars.next();
                }
            }
            '>' => {
                tokens.push(Token {
                    t_value: String::from(c),
                    t_type: IsiToken::RARROW,
                    t_line: app.line_count,
                    t_column: app.column_count,
                });
                chars.next();
            }
            '<' => {
                tokens.push(Token {
                    t_value: String::from(c),
                    t_type: IsiToken::LARROW,
                    t_line: app.line_count,
                    t_column: app.column_count,
                });
                chars.next();
            }
            '(' => {
                tokens.push(Token {
                    t_value: String::from(c),
                    t_type: IsiToken::LPAREN,
                    t_line: app.line_count,
                    t_column: app.column_count,
                });
                chars.next();
            }
            ')' => {
                tokens.push(Token {
                    t_value: String::from(c),
                    t_type: IsiToken::RPAREN,
                    t_line: app.line_count,
                    t_column: app.column_count,
                });
                chars.next();
            }
            '[' => {
                tokens.push(Token {
                    t_value: String::from(c),
                    t_type: IsiToken::LBRACKET,
                    t_line: app.line_count,
                    t_column: app.column_count,
                });
                chars.next();
            }
            ']' => {
                tokens.push(Token {
                    t_value: String::from(c),
                    t_type: IsiToken::RBRACKET,
                    t_line: app.line_count,
                    t_column: app.column_count,
                });
                chars.next();
            }
            _ => {
                if c.is_whitespace() || c == '\r' {
                    app.column_count += 1;
                    chars.next();
                } else if c == '\n' {
                    app.line_count += 1;
                    app.column_count = 0;
                    chars.next();
                } else {
                    print_compile_error(format!("Unknown token: `{}`", c));
                }
            }
        }
    }

    return tokens;
}
